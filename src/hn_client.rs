use hyper;
use hyper::rt::{Future, Stream};
use hyper::Body;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json;
use std::path::Display;
use std::str;
use tokio::runtime::Runtime;
use std::io::Cursor;

const HN_API_URL_TOPSTORIES: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

pub fn get_raw_from_url(url: &str) -> Result<Vec<u8>, String> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, Body>(https);

    let url = url.parse::<hyper::Uri>().map_err(|e| e.to_string())?;

    let request = client.get(url).and_then(|res| res.into_body().concat2());

    let mut runtime = Runtime::new().unwrap();
    runtime
        .block_on(request)
        .map_err(|e| e.to_string())
        .map(|r| r.to_vec())
}

pub fn get_from_url<T>(url: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    get_raw_from_url(url)
        .and_then(|r| serde_json::from_reader(Cursor::new(&r)).map_err(|e| e.to_string()))
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
    fn get_raw_from_url_test() {
        get_raw_from_url(HN_API_URL_TOPSTORIES)
            .and_then(|r| {
                str::from_utf8(&r)
                    .map(|r| r.to_string())
                    .map_err(|e| e.to_string())
            })
            .map(|r| println!("Result: {:?}", r))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn get_raw_from_url_should_panic_for_invalid_url() {
        get_raw_from_url("haha").unwrap();
    }

    #[test]
    fn de_list_test() {
        let data = "[17915371,17915560,17914959,17915522,17914723,17915487,17914731,17912734,17914935,17914634,17907816]";

        let list: Vec<u64> = serde_json::from_str(&data).unwrap();

        println!("{:?}", list);
    }

    #[test]
    fn get_from_url_test() {
        get_from_url(HN_API_URL_TOPSTORIES)
            .map(|r: Vec<u64>| println!("Result: {:?}", r))
            .unwrap();
    }
}
