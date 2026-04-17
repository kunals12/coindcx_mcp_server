use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub coindcx_api_key: String,
    pub coindcx_secret_key: String,
}

impl Config {
    pub fn init() -> Self {
        dotenv().ok();

        let coindcx_api_key = env::var("COINDCX_API_KEY").expect("COINDCX_API_KEY must be set");
        let coindcx_secret_key =
            env::var("COINDCX_SECRET_KEY").expect("COINDCX_SECRET_KEY must be set");

        Config {
            coindcx_api_key,
            coindcx_secret_key,
        }
    }
}
