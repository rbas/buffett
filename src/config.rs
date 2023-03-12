use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub pushover: PushOverConf,
    pub db: DbConf,
}
#[derive(Deserialize, Debug)]
pub struct PushOverConf {
    pub api_url: String,
    pub app_token: String,
    pub delivery_group_token: String,
}

#[derive(Deserialize, Debug)]
pub struct DbConf {
    pub path: String,
}

pub fn load_config(file_path: &str) -> Config {
    let config_data =
        fs::read_to_string(file_path).expect(format!("Cannot read file `{}`. Please create configuration file from template `conf_template.toml`.", file_path).as_str());
    let config: Config = toml::from_str(&config_data).unwrap();

    config
}
