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

pub struct HNApiUrl;

impl HNApiUrl {
    #[allow(non_snake_case)]
    fn HN_API_URL_TOPSTORIES() -> &'static str {
        "https://hacker-news.firebaseio.com/v0/topstories.json"
    }

    #[allow(non_snake_case)]
    fn HN_API_URL_ITEM(id: u64) -> String {
        format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id)
    }
}

pub struct HNClient {
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl HNClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, Body>(https);

        HNClient { client }
    }

    fn get_from_url<T>(&self, url: &str) -> Result<T, failure::Error>
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

    pub fn get_top_list(&self) -> Result<Vec<u64>, failure::Error> {
        self.get_from_url::<Vec<u64>>(HNApiUrl::HN_API_URL_TOPSTORIES())
    }

    pub fn get_item(&self, id: u64) -> Result<HNItem, failure::Error> {
        self.get_from_url::<HNItem>(&HNApiUrl::HN_API_URL_ITEM(id))
    }
}

#[derive(Deserialize, Debug)]
pub enum HNItemType {
    #[serde(rename = "job")]
    Job,
    #[serde(rename = "story")]
    Story,
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "poll")]
    Poll,
    #[serde(rename = "pollopt")]
    PollOpt,
}

#[derive(Deserialize, Debug)]
pub struct HNItem {
    id: u64,
    by: String,
    kids: Vec<u64>,
    title: String,

    #[serde(rename = "type")]
    item_type: HNItemType,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn top_story_list_test() {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, Body>(https);

        let uri = HNApiUrl::HN_API_URL_TOPSTORIES().parse().unwrap();

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
    fn get_top_list_test() {
        let client = HNClient::new();

        match client.get_top_list() {
            Ok(r) => {
                r.iter().take(3)
                    .flat_map(|&id| client.get_item(id))
                    .map(|item| println!("{}", item.title))
                    .collect()
            },
            Err(e) => panic!(e)
        }
    }


    #[test]
    fn get_item_test() {
        let client = HNClient::new();

        match client.get_item(17915371) {
            Ok(r) => {
                println!("{:?}", r);
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
