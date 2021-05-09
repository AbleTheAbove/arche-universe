use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub vsync: Option<bool>,
}
impl Default for Config {
    fn default() -> Config {
        Config { vsync: Some(false) }
    }
}

pub fn load_config() -> Config {
    let toml_str = r#""#;

    let decoded: Config = toml::from_str(toml_str).unwrap();
    return decoded;
}
