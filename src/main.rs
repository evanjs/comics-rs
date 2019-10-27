use ozone::{Config, ComicClient};
use envy;
use dotenv;

fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    dotenv::dotenv().ok();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error)
    };

    let comic_client: ComicClient = ComicClient::new(config);
    // 31 - Batman
    // how do I wrap this into a more generic error type?
    let series = comic_client.get_series(31).expect("failed to get series");
    println!("{:#?}", series);
    Ok(())
}
