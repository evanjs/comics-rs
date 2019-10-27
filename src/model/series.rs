#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    error: String,
    limit: u64,
    offset: u64,
    number_of_page_results: u64,
    number_of_total_results: u64,
    status_code: u64,
    results: Results,
    version: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Results {
    aliases: ::serde_json::Value,
    api_detail_url: String,
    characters: Vec<Character>,
    count_of_episodes: u64,
    date_added: String,
    date_last_updated: String,
    deck: ::serde_json::Value,
    description: ::serde_json::Value,
    episodes: Vec<Episode>,
    first_episode: FirstEpisode,
    id: u64,
    image: Image,
    last_episode: LastEpisode,
    name: String,
    publisher: Publisher,
    site_detail_url: String,
    start_year: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Character {
    api_detail_url: String,
    id: u64,
    name: String,
    site_detail_url: String,
    count: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Episode {
    api_detail_url: String,
    id: u64,
    name: String,
    site_detail_url: String,
    episode_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct FirstEpisode {
    api_detail_url: String,
    id: u64,
    name: String,
    episode_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct LastEpisode {
    api_detail_url: String,
    id: u64,
    name: String,
    episode_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Publisher {
    api_detail_url: String,
    id: u64,
    name: String,
}
