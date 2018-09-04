use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper::Body;
use hyper;
use tokio::runtime::Runtime;
use hyper_tls::HttpsConnector;
use std::io::{self, Write};

const HN_API_URL_TOPSTORIES: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

#[cfg(test)]
mod test {
    use super::*;
    use std::str;

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
}
