use failure;
use hyper;
use hyper::client::HttpConnector;
use hyper::rt::{Future, Stream};
use hyper::Body;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json;
use std::io::Cursor;
use std::str;
use tokio::runtime::Runtime;

const HN_API_URL_TOPSTORIES: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

pub struct HNClient {
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl HNClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, Body>(https);

        HNClient { client }
    }

    pub fn get_from_url<T>(&self, url: &str) -> Result<T, failure::Error>
    where
        T: DeserializeOwned,
    {
        let url = url.parse::<hyper::Uri>()?;

        let request = self
            .client
            .get(url)
            .and_then(|res| res.into_body().concat2());

        let mut runtime = Runtime::new().unwrap();
        let response = runtime.block_on(request)?;

        let response = serde_json::from_reader(Cursor::new(response))?;

        Ok(response)
    }
}

pub fn get_from_url<T>(url: &str) -> Result<T, failure::Error>
where
    T: DeserializeOwned,
{
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, Body>(https);

    let url = url.parse::<hyper::Uri>()?;

    let request = client.get(url).and_then(|res| res.into_body().concat2());

    let mut runtime = Runtime::new().unwrap();
    let response = runtime.block_on(request)?;

    let response = serde_json::from_reader(Cursor::new(response))?;

    Ok(response)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn top_story_list_test() {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, Body>(https);

        let uri = HN_API_URL_TOPSTORIES.parse().unwrap();

        let request = client
            .get(uri)
            .and_then(|res| res.into_body().concat2())
            .map(|res| {
                println!("{}", str::from_utf8(&res.to_vec()).unwrap());
            })
            .map_err(|err| {
                println!("Error: {}", err);
            });

        let mut runtime = Runtime::new().unwrap();
        match runtime.block_on(request) {
            Ok(s) => {
                println!("After wait: Result: {:#?}", &s);
            }
            Err(_) => (),
        };
    }

    #[test]
    fn de_list_test() {
        let data = "[17915371,17915560,17914959,17915522,17914723,17915487,17914731,17912734,17914935,17914634,17907816]";

        let list: Vec<u64> = serde_json::from_str(&data).unwrap();

        println!("{:?}", list);
    }

    #[test]
    fn get_from_url_test() {
        let client = HNClient::new();

        match client.get_from_url::<Vec<u64>>(HN_API_URL_TOPSTORIES) {
            Ok(r) => {
                for id in r.iter() {
                    println!("{}", id);
                }
            },
            Err(e) => panic!(e)
        }
    }

    #[test]
    #[should_panic]
    fn get_from_url_should_panic_for_invalid_url() {
        let client = HNClient::new();

        client.get_from_url::<Vec<u64>>("haha").unwrap();
    }
}
