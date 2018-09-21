extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

extern crate failure;
#[macro_use] extern crate failure_derive;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate futures;

pub mod hn_client;
pub mod word_count;
pub mod ownership_demo;
pub mod traitsamples;