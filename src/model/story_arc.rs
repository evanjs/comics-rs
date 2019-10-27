
use failure::_core::fmt::{Error, Formatter};
use std::fmt::Display;
use std::option::Option;
use std::option::Option::Some;
use std::prelude::v1::Vec;
use std::result::Result;
use std::result::Result::Ok;
use std::string::String;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
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

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, Default, Clone, PartialEq)]
pub struct Issues(Vec<Issue>);

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Results {
    aliases: ::serde_json::Value,
    api_detail_url: String,
    count_of_isssue_appearances: u64,
    // TODO: DateTime<Utc>?
    date_added: String,
    // TODO: DateTime<Utc>?
    date_last_updated: String,
    deck: String,
    description: Option<String>,
    episodes: Option<Vec<::serde_json::Value>>,
    first_appeared_in_episode: Option<::serde_json::Value>,
    first_appeared_in_issue: Option<FirstAppearedInIssue>,
    pub id: u64,
    image: Option<Image>,
    pub issues: Issues,
    movies: Option<Vec<::serde_json::Value>>,
    name: String,
    publisher: Option<Publisher>,
    site_detail_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct FirstAppearedInIssue {
    api_detail_url: String,
    id: u64,
    name: String,
    issue_number: String,
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
pub struct Issue {
    api_detail_url: String,
    pub id: u64,
    pub name: Option<String>,
    site_detail_url: String,
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Publisher {
    api_detail_url: String,
    id: u64,
    name: String,
    site_detail_url: String,
}
