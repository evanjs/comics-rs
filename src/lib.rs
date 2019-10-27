use error_chain::error_chain;
use failure::{AsFail, Fail};
use reqwest;
use reqwest::Method;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer};
use std::prelude::v1::{Box, Vec};
use std::string::{String, ToString};

mod model;

use crate::{
    model::story_arcs::Root as StoryArcs,
    model::series::Root as Series,
    model::serie::Root as Serie,
    model::story_arc::Root as StoryArc,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::io::Read;
use std::option::Option;
use std::option::Option::{None, Some};

#[macro_use]
extern crate tracing;

use std::result::Result::Ok;
use std::mem::drop;
use failure::_core::result::Result::Err;
use failure::Error;
use std::clone::Clone;

type Result<T> = std::result::Result<T, failure::Error>;


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

impl ComicClient {
    pub fn new(config: Config) -> Self {
        ComicClient {
            client: reqwest::Client::new(),
            config,
            base_url: Url::parse(&BASE_URL).unwrap(),
        }
    }

    pub fn get_story_arc(&self, story_arc_id: u64) -> Result<StoryArc> {
        let result: Result<StoryArc> = self.get_resource(Some(story_arc_id), "story_arc", None, None, None);
        result
    }

    pub fn search_series(&self, query: &str) -> Result<Serie> {
        let things: Series = self.get_resource(None, "series", None, Some(query), Some("series"))?;
        match things.results.first() {
            Some(s) => {
                self.get_series(s.id.clone())
            }
            None => {
                Err(failure::err_msg(format!("no matches found for {}", query)))
            }
        }
    }

    pub fn search_story_arc(&self, query: &str) -> Result<StoryArc> {
        let filter = format!("name:{}", query);
        let things: StoryArcs = self.get_resource(None, "story_arcs", Some(&filter), None, None)?;
        match things.results.first() {
            Some(s) => {
                self.get_story_arc(s.id.clone())
            }
            None => {
                Err(failure::err_msg(format!("no matches found for {}", query)))
            }
        }
    }

    pub fn get_series(&self, series_id: u64) -> Result<Serie> {
        self.get_resource(Some(series_id), &"series".to_string(), None, None, None)
    }

    pub fn get_resource<T: DeserializeOwned>(
        &self,
        id: Option<u64>,
        resource_name: &str,
        filter: Option<&str>,
        query: Option<&str>,
        resources: Option<&str>,
    ) -> Result<T> {
        let mut full_url = self.base_url.clone();
        let mut query_map = HashMap::new();
        query_map.insert("format", "json");
        query_map.insert("api_key", &self.config.api_key);
        match resources {
            None => {
                full_url = full_url
                    .join(format!("{}/", resource_name).as_str())
                    .expect("failed to join resource/ onto base url");
            }
            Some(r) => {
                full_url = full_url
                    .join("search/")?;
                query_map.insert("resources", r);
            }
        }
        match id {
            Some(i) => full_url = full_url.join(&i.to_string()).expect("Failed to add on id"),
            _ => {}
        }
        dbg!(&full_url);

        match filter {
            Some(f) => drop(query_map.insert("filter", f)),
            None => {}
        }
        match query {
            Some(q) => drop(query_map.insert("query", q)),
            _ => {}
        }
        let mut request = self.client.request(Method::GET, full_url).query(&query_map);
        debug!("Full request: {:#?}", &request);
        let mut response = request.send()?;
        let json = response.json()?;
        let resource: T = serde_json::from_value(json)?;
        Ok(resource)
    }
}
