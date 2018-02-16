use std::iter::FromIterator;

use rincon_core::api::auth::{Authentication, Credentials};
use rincon_core::api::method::{Parameters, Prepare};
use rincon_core::api::user_agent::Version;
use super::*;

struct Prepared<'a> {
    operation: Operation,
    path: &'a str,
    params: Vec<(&'a str, &'a str)>,
    content: Option<Value>
}

impl<'a> Prepare for Prepared<'a> {
    type Content = Value;

    fn operation(&self) -> Operation {
        self.operation.clone()
    }

    fn path(&self) -> String {
        String::from(self.path)
    }

    fn parameters(&self) -> Parameters {
        Parameters::from_iter(self.params.iter())
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        self.content.as_ref()
    }
}

#[test]
fn build_request_uri_for_http() {
    let datasource = DataSource::from_url("http://localhost:8529").unwrap();
    let prepared = Prepared {
        operation: Operation::Read,
        path: "/_api/user",
        params: vec![],
        content: None,
    };

    let uri = build_request_uri(&datasource, None, &prepared);

    assert_eq!("http://localhost:8529/_api/user", uri.to_string());
}

#[test]
fn build_request_uri_for_https_with_authentication() {
    let datasource = DataSource::from_url("https://localhost:8529").unwrap()
        .with_authentication(Authentication::Basic(
            Credentials::new("micky".to_owned(), "pass".to_owned())));
    let prepared = Prepared {
        operation: Operation::Read,
        path: "/_api/user",
        params: vec![],
        content: None,
    };

    let uri = build_request_uri(&datasource, None, &prepared);

    assert_eq!("https://localhost:8529/_api/user", uri.to_string());
}

#[test]
fn build_request_uri_for_given_database() {
    let datasource = DataSource::from_url("https://localhost:8529").unwrap()
        .use_database("url_test");
    let prepared = Prepared {
        operation: Operation::Read,
        path: "/_api/collection",
        params: vec![],
        content: None,
    };

    let uri = build_request_uri(&datasource, Some(&"given_db_name".to_owned()), &prepared);

    assert_eq!("https://localhost:8529/_db/given_db_name/_api/collection", uri.to_string());
}

#[test]
fn build_request_uri_for_specific_database() {
    let datasource = DataSource::from_url("https://localhost:8529").unwrap()
        .use_database("url_test");
    let prepared = Prepared {
        operation: Operation::Read,
        path: "/_api/collection",
        params: vec![],
        content: None,
    };

    let uri = build_request_uri(&datasource, datasource.database_name(), &prepared);

    assert_eq!("https://localhost:8529/_db/url_test/_api/collection", uri.to_string());
}

#[test]
fn build_request_uri_for_specific_database_with_one_param() {
    let datasource = DataSource::from_url("https://localhost:8529").unwrap()
        .use_database("the big data");
    let prepared = Prepared {
        operation: Operation::Read,
        path: "/_api/document",
        params: vec![("id", "25")],
        content: None,
    };

    let uri = build_request_uri(&datasource, datasource.database_name(), &prepared);

    assert_eq!("https://localhost:8529/_db/the%20big%20data/_api/document\
                ?id=25", uri.to_string());
}

#[test]
fn build_request_uri_for_specific_database_with_two_params() {
    let datasource = DataSource::from_url("https://localhost:8529").unwrap()
        .use_database("the büg data");
    let prepared = Prepared {
        operation: Operation::Read,
        path: "/_api/document",
        params: vec![("id", "25"), ("name", "JuneReport")],
        content: None,
    };

    let uri = build_request_uri(&datasource, datasource.database_name(), &prepared);

    assert_eq!("https://localhost:8529/_db/the%20b%C3%BCg%20data/_api/document\
                ?id=25&name=JuneReport", uri.to_string());
}

#[test]
fn build_request_uri_for_given_database_with_three_params() {
    let datasource = DataSource::from_url("https://localhost:8529").unwrap()
        .use_database("default_test_database");
    let prepared = Prepared {
        operation: Operation::Read,
        path: "/_api/document",
        params: vec![("id", "25"), ("name", "JuneReport"), ("max", "42")],
        content: None,
    };

    let uri = build_request_uri(&datasource, Some(&"the big data".to_owned()), &prepared);

    assert_eq!("https://localhost:8529/_db/the%20big%20data/_api/document\
                ?id=25&name=JuneReport&max=42", uri.to_string());
}

#[test]
fn header_user_agent_for_my_user_agent() {
    #[derive(Debug)]
    struct MyUserAgent;
    #[derive(Debug)]
    struct MyVersion;

    impl UserAgent for MyUserAgent {
        fn name(&self) -> &str {
            "rincon"
        }

        fn version(&self) -> &Version {
            &MyVersion
        }

        fn homepage(&self) -> &str {
            "https://github.com/innoave/rincon"
        }
    }

    impl Version for MyVersion {
        fn major(&self) -> &str {
            "2"
        }

        fn minor(&self) -> &str {
            "5"
        }

        fn patch(&self) -> &str {
            "9"
        }

        fn pre(&self) -> &str {
            ""
        }
    }

    let agent = header_user_agent_for(&MyUserAgent);

    assert_eq!(
        header::UserAgent::new("Mozilla/5.0 (compatible; rincon/2.5; +https://github.com/innoave/rincon)"),
        agent
    );
}
