#[macro_use]
extern crate clap;

use clap::App;

use dotenv;
use envy;
use ozone::{ComicClient, Config};

use std::result::Result;
use std::result::Result::{Err, Ok};

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    dotenv::dotenv().ok();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let arc = value_t!(matches.value_of("arc"), String).unwrap_or("".to_string());
    let series = value_t!(matches.value_of("series"), String).unwrap_or("".to_string());

    let comic_client: ComicClient = ComicClient::new(config);
    if arc != "" {
        match comic_client.search_story_arc(&arc) {
            Ok(arc_result) => println!("{}", arc_result.results.issues),
            Err(e) => println!("{}", e),
        }
    }

    if series != "" {
        match comic_client.search_series(&series) {
            Ok(arc_result) => println!("{:#?}", arc_result.results),
            Err(e) => println!("{}", e),
        }
    }
    Ok(())
}
