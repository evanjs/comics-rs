use chrono::prelude::*;
use failure::_core::fmt::{Error, Formatter};
use std::fmt::Display;
use std::option::Option;
use std::option::Option::Some;
use std::prelude::v1::Vec;
use std::result::Result;
use std::result::Result::Ok;
use std::string::String;
use crate::deserializer;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Root {
    error: String,
    limit: u64,
    offset: u64,
    number_of_page_results: u64,
    number_of_total_results: u64,
    status_code: u64,
    pub results: Results,
    version: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone, PartialEq)]
pub struct Issues(Vec<Issue>);

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Results {
    aliases: ::serde_json::Value,
    api_detail_url: Option<String>,
    count_of_isssue_appearances: u64,
    #[serde(deserialize_with = "deserializer::deserialize_optional_datetime")]
    date_added: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserializer::deserialize_optional_datetime")]
    date_last_updated: Option<DateTime<Utc>>,
    deck: Option<String>,
    description: Option<String>,
    episodes: Option<Vec<::serde_json::Value>>,
    first_appeared_in_episode: Option<::serde_json::Value>,
    first_appeared_in_issue: Option<FirstAppearedInIssue>,
    pub id: u64,
    image: Option<Image>,
    pub issues: Issues,
    movies: Option<Vec<::serde_json::Value>>,
    name: Option<String>,
    publisher: Option<Publisher>,
    site_detail_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct FirstAppearedInIssue {
    api_detail_url: Option<String>,
    id: u64,
    name: Option<String>,
    issue_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Image {
    icon_url: Option<String>,
    medium_url: Option<String>,
    screen_url: Option<String>,
    screen_large_url: Option<String>,
    small_url: Option<String>,
    super_url: Option<String>,
    thumb_url: Option<String>,
    tiny_url: Option<String>,
    original_url: Option<String>,
    image_tags: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Issue {
    api_detail_url: Option<String>,
    pub id: u64,
    pub name: Option<String>,
    site_detail_url: Option<String>,
}

impl Display for Issues {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Issues:\n")?;
        for v in &self.0 {
            writeln!(f, "{}", v)?;
        }
        Ok(())
    }
}

impl Display for Issue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.name {
            Some(name) => {
                // TODO - add left padding
                write!(f, "{} - URL: {}", name, self.site_detail_url)
            }
            None => write!(
                f,
                "No name for issue {} - URL: {}",
                self.id, self.site_detail_url
            ),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Publisher {
    api_detail_url: Option<String>,
    id: u64,
    name: Option<String>,
    site_detail_url: Option<String>,
}
