use std::fs::{self};

//imports
use serde::{Deserialize, Serialize};

//derive the traits for your struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub markets: Vec<Vec<String>>,
}

impl Config {
    pub fn load_config(file_path: &str) -> Config {
        //This function loads the config from the config.json file into the app. Can Panic here. The app will not start if the config is not loaded

        let file = fs::File::open(file_path).unwrap_or_else(|error| {
            panic!("Error while opening the config file. Error {}", error);
        });

        let config_parsed_from_json: Config =
            serde_json::from_reader(file).unwrap_or_else(|error| {
                panic!(
                    "Error while parsing the config file into JSON. Error {}",
                    error
                );
            });

        config_parsed_from_json
    }
}
