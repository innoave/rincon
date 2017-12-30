
#![doc(html_root_url = "https://docs.rs/rincon_session_async/0.1.0")]

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
)]

//#[cfg(test)] #[macro_use] extern crate hamcrest;

extern crate futures;
extern crate serde;

extern crate rincon_core;
extern crate rincon_client;

mod async;
pub use self::async::*;
