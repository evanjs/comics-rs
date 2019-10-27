use url::Url;
use chrono::{DateTime, Utc};
use std::string::String;
use std::option::Option;
use std::prelude::v1::Vec;
use serde::{Deserialize, Serialize};
use serde_derive;
use std::collections::{HashMap, HashSet};

// Move this elsewhere, maybe filters.rs?
enum Format {
    Xml,
    Json,
    Jsonp,
}

#[derive(Deserialize, Serialize, Debug)]
struct EpisodeCredit {
    api_detail_url: Url,
    id: u64,
    name: String,
    episode_number: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Image {
    icon_url: Url,
    medium_url: Url,
    screen_url: Url,
    screen_large_url: Url,
    small_url: Url,
    super_url: Url,
    thumb_url: Url,
    tiny_url: Url,
    original_url: Url,
    image_tags: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct PublisherCredit {
    api_detail_url: Url,
    id: u64,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CharacterCredit {
    api_detail_url: Url,
    id: u64,
    name: String,
    site_detail_url: Url,
    count: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ComicRequest<T> {
    error: String,
    limit: u8,
    offset: u64,
    number_of_page_results: u64,
    number_of_total_results: u64,
    status_code: u16,
    results:
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Series {
    // separated by newlines
    // maybe vectorize it with a get method, etc?
    aliases: Option<String>,
    api_detail_url: Url,
    // List of characters that appear in the series
    // is Item=String okay here?
    character_credits: Vec<CharacterCredit>,
    // Is this an appropriate number type?
    count_of_episodes: u64,
    date_added: DateTime<Utc>,
    date_last_updated: DateTime<Utc>,
    deck: Option<String>,
    // This contains HTML
    description: String,
    episodes: Vec<EpisodeCredit>,
    first_episode: EpisodeCredit,
    id: u64,
    image: Url,
    last_episode: EpisodeCredit,
    location_credits: String,
    name: String,
    publisher: PublisherCredit,
    site_detail_url: Url,
    start_year: String,
}
