use std::option::Option;
use std::prelude::v1::Vec;
use std::string::String;
use chrono::{DateTime, Utc};
use crate::deserializer;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Root {
    error: String,
    limit: u64,
    offset: u64,
    number_of_page_results: u64,
    number_of_total_results: u64,
    status_code: u64,
    pub results: Vec<Results>,
    version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Results {
    aliases: ::serde_json::Value,
    api_detail_url: String,
    characters: Option<Vec<Character>>,
    count_of_episodes: u64,
    #[serde(deserialize_with = "deserializer::deserialize_optional_datetime")]
    date_added: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserializer::deserialize_optional_datetime")]
    date_last_updated: Option<DateTime<Utc>>,
    deck: ::serde_json::Value,
    description: ::serde_json::Value,
    episodes: Option<Vec<Episode>>,
    first_episode: Option<FirstEpisode>,
    pub id: u64,
    image: Image,
    last_episode: Option<LastEpisode>,
    name: String,
    publisher: Option<Publisher>,
    site_detail_url: Option<String>,
    start_year: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Character {
    api_detail_url: String,
    id: u64,
    name: String,
    site_detail_url: String,
    count: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Episode {
    api_detail_url: String,
    id: u64,
    name: String,
    site_detail_url: String,
    episode_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct FirstEpisode {
    api_detail_url: String,
    id: u64,
    name: String,
    episode_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Image {
    icon_url: String,
    medium_url: String,
    screen_url: String,
    screen_large_url: String,
    small_url: String,
    super_url: String,
    thumb_url: String,
    tiny_url: String,
    original_url: String,
    image_tags: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct LastEpisode {
    api_detail_url: String,
    id: u64,
    name: String,
    episode_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Publisher {
    api_detail_url: String,
    id: u64,
    name: String,
}
