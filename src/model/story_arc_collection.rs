use std::option::Option;
use std::prelude::v1::Vec;
use std::string::String;
use chrono::{DateTime, Utc};
use crate::deserializer;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    error: Option<String>,
    limit: u64,
    offset: u64,
    number_of_page_results: u64,
    number_of_total_results: u64,
    status_code: u64,
    pub results: Vec<Result>,
    version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Result {
    aliases: ::serde_json::Value,
    api_detail_url: Option<String>,
    count_of_isssue_appearances: u64,
    #[serde(deserialize_with = "deserializer::deserialize_optional_datetime")]
    date_added: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserializer::deserialize_optional_datetime")]
    date_last_updated: Option<DateTime<Utc>>,
    deck: Option<String>,
    description: Option<String>,
    first_appeared_in_issue: Option<FirstAppearedInIssue>,
    first_appeared_in_episode: Option<::serde_json::Value>,
    pub id: u64,
    image: Image,
    name: Option<String>,
    publisher: Publisher,
    site_detail_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct FirstAppearedInIssue {
    api_detail_url: Option<String>,
    id: u64,
    name: Option<String>,
    issue_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Publisher {
    api_detail_url: Option<String>,
    id: u64,
    name: Option<String>,
    site_detail_url: Option<String>,
}
