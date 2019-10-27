use reqwest;
use serde::Deserialize;
use serde_json::{Deserializer, Serializer};
use reqwest::Url;
use reqwest::Method;
use failure::{Fail, AsFail};
use std::string::{ToString, String};
use std::prelude::v1::Box;
use error_chain::error_chain;

mod model;

use crate::model::series::Root as Series;
use std::io::Read;
use tracing::log::debug;

const BASE_URL: &str = "https://comicvine.gamespot.com/api/";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
}

pub struct ComicClient {
    pub client: reqwest::Client,
    pub config: Config,
    pub base_url: Url,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        Serde(serde_json::error::Error);
    }

    errors { ComicClientResponseError(t: String) }
}

fn parse_response(mut response: reqwest::Response) -> Result<String> {
    let mut body = String::new();
    response.read_to_string(&mut body)?;
    body.parse::<String>()
        .chain_err(|| ErrorKind::ComicClientResponseError(body))
}


impl ComicClient {
    pub fn new(config: Config) -> Self {
        ComicClient {
            client: reqwest::Client::new(),
            config,
            base_url: Url::parse(&BASE_URL).unwrap(),
        }
    }


    pub fn get_series(&self, series_id: u64) -> Result<Series> {
        let full_url = self.base_url.join("series/").expect("failed to join series/ onto base url").join(&series_id.to_string()).expect("Failed to add on series id");
        dbg!(&full_url);
        let request = self.client.request(Method::GET, full_url).query(&[("format", "json"), ("api_key", &self.config.api_key)]);
        let response = request.send()?;
        let parsed = parse_response(response)?;
        let series: Series = serde_json::from_str(&parsed.to_string())?;
        Ok(series)
    }
}
