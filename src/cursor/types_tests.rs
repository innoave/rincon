
use serde_json;

use api::query::Query;
use api::types::Value;
use aql::OptimizerRule;
use super::types::*;

#[test]
fn convert_query_into_new_cursor_to_be_created() {
    let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
    query.set_parameter("name", "simone");
    let query = query;

    let new_cursor: NewCursor = query.into();

    assert_eq!("FOR u IN users FILTER u.name = @name RETURN u.name", new_cursor.query());
    assert_eq!(Some(&Value::String("simone".to_owned())), new_cursor.bind_vars().get("name"));
}

#[test]
fn set_optimizer_rule_cursor_option_on_a_newly_initialized_new_cursor() {
    let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
    query.set_parameter("name".to_owned(), "simone");
    let query = query;

    let mut new_cursor = NewCursor::from(query);
    assert!(new_cursor.options().is_none());

    new_cursor.options_mut().optimizer_mut().rules_mut()
        .include(OptimizerRule::UseIndexes)
        .exclude(OptimizerRule::MoveFiltersUp);
    let new_cursor = new_cursor;

    assert!(new_cursor.options().is_some());
    assert!(new_cursor.options().unwrap().optimizer().is_some());

    let optimizer_rules = new_cursor.options().unwrap().optimizer().unwrap().rules();
    assert!(optimizer_rules.includes(&OptimizerRule::UseIndexes));
    assert!(optimizer_rules.excludes(&OptimizerRule::MoveFiltersUp));
}

#[test]
fn set_cursor_options_on_newly_initialized_new_cursor() {
    let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
    query.set_parameter("name", "simone".to_owned());
    let query = query;

    let mut new_cursor = NewCursor::from(query);
    assert!(new_cursor.options().is_none());

    {
        let cursor_options = new_cursor.options_mut();
        cursor_options.set_fail_on_warning(true);
        cursor_options.set_full_count(Some(false));
        cursor_options.set_max_warning_count(None);
        cursor_options.set_max_plans(5);

        #[cfg(feature = "rocksdb")] {
            cursor_options.set_intermediate_commit_count(1);
        }
        #[cfg(feature = "enterprise")] {
            cursor_options.set_satellite_sync_wait(false);
        }
    }
    let new_cursor = new_cursor;
    assert!(new_cursor.options().is_some());
    let cursor_options = new_cursor.options().unwrap();

    assert_eq!(Some(true), cursor_options.is_fail_on_warning());
    assert_eq!(None, cursor_options.is_profile());
    assert_eq!(None, cursor_options.max_warning_count());
    assert_eq!(Some(false), cursor_options.is_full_count());
    assert_eq!(Some(5), cursor_options.max_plans());

    #[cfg(feature = "rocksdb")] {
        assert_eq!(Some(1), cursor_options.intermediate_commit_count());
        assert_eq!(None, cursor_options.intermediate_commit_size());
        assert_eq!(None, cursor_options.max_transaction_size());
    }
    #[cfg(feature = "enterprise")] {
        assert_eq!(Some(false), cursor_options.satellite_sync_wait());
    }
}

#[test]
fn set_options_on_newly_initialized_new_cursor() {
    let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
    query.set_parameter("name".to_owned(), "simone".to_owned());
    let query = query;

    let mut new_cursor = NewCursor::from(query);
    new_cursor.set_batch_size(42);
    new_cursor.set_cache(false);
    new_cursor.set_count(None);
    new_cursor.set_memory_limit(32 * 1024);
    new_cursor.set_ttl(Some(30));
    let new_cursor = new_cursor;

    assert_eq!(Some(42), new_cursor.batch_size());
    assert_eq!(Some(false), new_cursor.is_cache());
    assert_eq!(None, new_cursor.is_count());
    assert_eq!(Some(32768), new_cursor.memory_limit());
    assert_eq!(Some(30), new_cursor.ttl());
}

#[test]
fn serialize_new_cursor_with_cursor_options_set() {
    let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
    query.set_parameter("name", "simone");
    let query = query;

    let mut new_cursor = NewCursor::from(query);
    assert!(new_cursor.options().is_none());

    {
        let cursor_options = new_cursor.options_mut();
        cursor_options.set_fail_on_warning(true);
        cursor_options.set_full_count(Some(false));
        cursor_options.set_max_warning_count(None);
        cursor_options.set_max_plans(5);

        #[cfg(feature = "rocksdb")] {
            cursor_options.set_intermediate_commit_count(1);
        }
        #[cfg(feature = "enterprise")] {
            cursor_options.set_satellite_sync_wait(false);
        }
    }
    let new_cursor = new_cursor;

    let json_cursor = serde_json::to_string(&new_cursor).unwrap();

    #[cfg(all(not(feature = "rocksdb"), not(feature = "enterprise")))] {
        assert_eq!(r#"{"query":"FOR u IN users FILTER u.name = @name RETURN u.name","bindVars":{"name":"simone"},"options":{"failOnWarning":true,"fullCount":false,"maxPlans":5}}"#, &json_cursor);
    }
    #[cfg(all(feature = "rocksdb", not(feature = "enterprise")))] {
        assert_eq!(r#"{"query":"FOR u IN users FILTER u.name = @name RETURN u.name","bindVars":{"name":"simone"},"options":{"failOnWarning":true,"fullCount":false,"maxPlans":5,"intermediateCommitCount":1}}"#, &json_cursor);
    }
    #[cfg(all(not(feature = "rocksdb"), feature = "enterprise"))] {
        assert_eq!(r#"{"query":"FOR u IN users FILTER u.name = @name RETURN u.name","bindVars":{"name":"simone"},"options":{"failOnWarning":true,"fullCount":false,"maxPlans":5,"satelliteSyncWait":false}}"#, &json_cursor);
    }
    #[cfg(all(feature = "rocksdb", feature = "enterprise"))] {
        assert_eq!(r#"{"query":"FOR u IN users FILTER u.name = @name RETURN u.name","bindVars":{"name":"simone"},"options":{"failOnWarning":true,"fullCount":false,"maxPlans":5,"intermediateCommitCount":1,"satelliteSyncWait":false}}"#, &json_cursor);
    }
}

#[test]
fn serialize_new_cursor_with_optimizer_rules_set() {
    let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
    query.set_parameter("name", "simone");
    let query = query;

    let mut new_cursor = NewCursor::from(query);
    assert!(new_cursor.options().is_none());

    new_cursor.options_mut().optimizer_mut().rules_mut()
        .exclude(OptimizerRule::All)
        .exclude(OptimizerRule::MoveFiltersUp)
        .include(OptimizerRule::UseIndexForSort)
        .include(OptimizerRule::InlineSubQueries)
    ;
    let new_cursor = new_cursor;

    let json_cursor = serde_json::to_string(&new_cursor).unwrap();

    assert!(json_cursor.starts_with(r#"{"query":"FOR u IN users FILTER u.name = @name RETURN u.name","bindVars":{"name":"simone"},"options":{"optimizer":{"rules":["#));
    assert!(json_cursor.ends_with(r#"]}}}"#));
    assert!(json_cursor.contains("-all"));
    assert!(json_cursor.contains("-move-filters-up"));
    assert!(json_cursor.contains("+use-index-for-sort"));
    assert!(json_cursor.contains("+inline-subqueries"));
}
