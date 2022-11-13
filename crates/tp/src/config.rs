use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct PathOptions {
    pub path: String,

    pub git: bool,

    pub ignore: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub paths: Vec<PathOptions>,
    pub ignore: Option<Vec<String>>,
}

pub(crate) fn get() -> Config {
    let config_file = fs::read_to_string("/home/branco/.config/tproject/config.json")
        .expect("Unable to read config file");

    serde_json::from_str(&config_file).expect("Invalid json format")
}
