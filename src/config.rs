use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub pushover: PushOverConf,
}
#[derive(Deserialize, Debug)]
pub struct PushOverConf {
    pub api_url: String,
    pub app_token: String,
    pub delivery_group_token: String,
}

pub fn load_config(file_path: &str) -> Config {
    let config_data =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let config: Config = toml::from_str(&config_data).unwrap();

    config
}
