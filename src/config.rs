use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Config {
    pub vsync: Option<bool>,
}

pub static DEFAULT_CONFIG: Config = Config { vsync: Some(false) };

pub fn load_config() -> Config {
    let filename = "config.toml";

    let contents = fs::read_to_string(filename);
    match contents {
        Ok(o) => {
            println!("{}", o);
            let decoded: Config = toml::from_str(&o).unwrap_or(DEFAULT_CONFIG);
            return decoded;
        }
        Err(e) => {
            println!("{}", e);
            return DEFAULT_CONFIG;
        }
    }
}
