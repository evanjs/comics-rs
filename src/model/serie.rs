#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    error: Option<String>,
    limit: u64,
    offset: u64,
    number_of_page_results: u64,
    number_of_total_results: u64,
    status_code: u64,
    pub results: Results,
    version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Results {
    aliases: ::serde_json::Value,
    api_detail_url: Option<String>,
    characters: Vec<Character>,
    count_of_episodes: u64,
    date_added: Option<String>,
    date_last_updated: Option<String>,
    deck: ::serde_json::Value,
    description: Option<String>,
    episodes: Vec<Episode>,
    first_episode: FirstEpisode,
    pub id: u64,
    image: Image,
    last_episode: LastEpisode,
    name: Option<String>,
    publisher: Publisher,
    site_detail_url: Option<String>,
    start_year: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Character {
    api_detail_url: Option<String>,
    id: u64,
    name: Option<String>,
    site_detail_url: Option<String>,
    count: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Episode {
    api_detail_url: Option<String>,
    id: u64,
    name: Option<String>,
    site_detail_url: Option<String>,
    episode_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct FirstEpisode {
    api_detail_url: Option<String>,
    id: u64,
    name: Option<String>,
    episode_number: Option<String>,
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
struct LastEpisode {
    api_detail_url: Option<String>,
    id: u64,
    name: Option<String>,
    episode_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Publisher {
    api_detail_url: Option<String>,
    id: u64,
    name: Option<String>,
}
