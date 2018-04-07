
use std::cell::RefCell;
use std::iter::{IntoIterator, Iterator};
use std::rc::Rc;
use std::vec::IntoIter;

use serde::de::DeserializeOwned;
use tokio_core::reactor::Core;

use rincon_client::cursor::methods::*;
use rincon_client::cursor::types::{Cursor, CursorStatistics, Warning};
use rincon_core::api::connector::{Connector, Execute};
use rincon_core::api::method::{Method, Prepare};
use rincon_core::api::types::{Empty, EMPTY};

use super::Result;

/// A session for operating with a specific `Cursor`.
#[derive(Debug)]
pub struct CursorSession<T, C> {
    cursor: Cursor<T>,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<T, C> CursorSession<T, C>
    where T: 'static + DeserializeOwned, C: 'static + Connector
{
    /// Instantiates a new `CursorSession` for the given cursor.
    pub(crate) fn new(
        cursor: Cursor<T>,
        database_name: String,
        connector: Rc<C>,
        core: Rc<RefCell<Core>>,
    ) -> Self {
        CursorSession {
            cursor,
            database_name,
            connector,
            core,
        }
    }

    /// Executes an API method applied to the database of this session.
    fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
        where M: 'static + Method + Prepare
    {
        self.core.borrow_mut().run(
            self.connector.connection(&self.database_name)
                .execute(method)
        )
    }

    /// Returns the name of the database the query has been executed for.
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Returns the `Cursor` entity of this session.
    pub fn entity(&self) -> &Cursor<T> {
        &self.cursor
    }

    /// Unwraps the `Cursor` entity out of this session.
    pub fn unwrap(self) -> Cursor<T> {
        self.cursor
    }

    /// Returns the id of this cursor.
    pub fn id(&self) -> Option<&String> {
        self.cursor.id()
    }

    /// Returns the slice of result documents retrieved with this cursor.
    ///
    /// The query may have more results. Whether a query has more results can
    /// be checked by the `has_more()` attribute function. To fetch the next
    /// batch of results use the `next_cursor()` function or iterate over all
    /// results by using the `Iterator` returned by the `into_iter()` function.
    pub fn result(&self) -> &[T] {
        self.cursor.result()
    }

    /// Returns whether there are more results available for this cursor on
    /// the server.
    pub fn has_more(&self) -> bool {
        self.cursor.has_more()
    }

    /// Returns whether the query result was served from the query cache or not.
    ///
    /// If the query result is served from the query cache, the stats attribute
    /// will be `None`.
    pub fn is_cached(&self) -> bool {
        self.cursor.is_cached()
    }

    /// Returns the total number of result documents available (only available
    /// if the query was executed with the count attribute set).
    pub fn count(&self) -> Option<u64> {
        self.cursor.count()
    }

    /// Returns the statistics about the execution of data modification queries.
    ///
    /// The stats will be `None` if the query is not a data modification query
    /// or the result is served from the query cache.
    pub fn stats(&self) -> Option<&CursorStatistics> {
        self.cursor.stats()
    }

    /// Returns warnings that occurred during query execution.
    pub fn warnings(&self) -> Option<&Vec<Warning>> {
        self.cursor.warnings()
    }

    /// Checks whether this cursor has more results and if yes fetches a
    /// cursor with the next batch of results and returns it as a new
    /// `CursorSession`.
    ///
    /// This function returns `None` if there are no more results for this
    /// cursor. It returns `Some(Error)` if fetching the next batch of results
    /// fails.
    pub fn next_cursor(&self) -> Option<Result<CursorSession<T, C>>> {
        self.cursor.id().map(|v| v.to_owned()).map(|id|
            self.execute(ReadNextBatchFromCursor::with_id(id))
                .map(|cursor| CursorSession {
                    cursor,
                    database_name: self.database_name.clone(),
                    connector: self.connector.clone(),
                    core: self.core.clone(),
                })
        )
    }

    /// Deletes the cursor represented by this session on the server if it is
    /// still existing otherwise those nothing.
    pub fn delete(self) -> Result<Empty> {
        if let Some(id) = self.cursor.id() {
            self.execute(DeleteCursor::with_id_ref(id))
        } else {
            Ok(EMPTY)
        }
    }
}

impl<T, C> IntoIterator for CursorSession<T, C>
    where T: 'static + DeserializeOwned, C: 'static + Connector
{
    type Item = Result<T>;
    type IntoIter = CursorSessionIntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        let has_more = self.cursor.has_more();
        let (cursor_id, count, result) = self.cursor.unwrap();
        CursorSessionIntoIter {
            batch: result.into_iter(),
            count,
            has_more,
            cursor_id,
            database_name: self.database_name,
            connector: self.connector,
            core: self.core,
        }
    }
}

/// An `Iterator` over all results for a specific cursor.
#[derive(Debug)]
pub struct CursorSessionIntoIter<T, C> {
    batch: IntoIter<T>,
    count: Option<u64>,
    has_more: bool,
    cursor_id: Option<String>,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<T, C> CursorSessionIntoIter<T, C>
    where T: 'static + DeserializeOwned, C: 'static + Connector
{
    /// Executes an API method applied to the database of this session.
    fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
        where M: 'static + Method + Prepare
    {
        self.core.borrow_mut().run(
            self.connector.connection(&self.database_name)
                .execute(method)
        )
    }
}

impl<T, C> Iterator for CursorSessionIntoIter<T, C>
    where T: 'static + DeserializeOwned, C: 'static + Connector
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.batch.next() {
            Some(Ok(next))
        } else if self.has_more {
            self.cursor_id.clone().and_then(|id| {
                let cursor = self.execute(ReadNextBatchFromCursor::new(id));
                match cursor {
                    Ok(cursor) => {
                        self.has_more = cursor.has_more();
                        let (id, count, result) = cursor.unwrap();
                        self.cursor_id = id;
                        self.count = count;
                        self.batch = result.into_iter();
                        self.batch.next().map(Ok)
                    },
                    Err(error) => Some(Err(error)),
                }
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.batch.len(), self.count.map(|c| c as usize))
    }

    fn count(self) -> usize where Self: Sized {
        self.count.map(|c| c as usize)
            .unwrap_or_else(|| self.fold(0, |cnt, _| cnt + 1))
    }
}
