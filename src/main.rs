#[macro_use]
extern crate clap;

use clap::App;

use dotenv;
use envy;
use ozone::{ComicClient, Config};
use std::result::Result;
use std::result::Result::{Ok, Err};
use std::iter::Iterator;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    dotenv::dotenv().ok();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let arc = value_t!(matches.value_of("arc"), String)?;

    let comic_client: ComicClient = ComicClient::new(config);
    // how do I wrap this into a more generic error type?
    match comic_client.search_story_arc(&arc) {
        Ok(arc_result) => println!("{}", arc_result.results.issues),
        Err(e) => {println!("{}", e)}
    }
    Ok(())
}
