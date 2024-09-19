use serde::Deserialize;
use std::env;
use std::fs::{self, File};
use std::io::{Error, Read};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub location: String,
    pub previous_version: String,
}

/// Reads frust project config files
///
/// # Returns
///
/// Returns a vector [`Vec<Config>`] with all frust project configurations
///
pub fn get_configs() -> Result<Vec<Config>, Error> {
    let home_path = format!("{}/.frust", env::var("HOME").unwrap());
    // check if folder exists, if not create .frust folder in home path
    if fs::metadata(&home_path).is_err() {
        fs::create_dir(&home_path).expect("error while creating .frust folder in home path");
    }
    let toml_paths = fs::read_dir(home_path)?;
    let mut return_paths: Vec<String> = vec![];
    let mut configs: Vec<Config> = vec![];

    for toml_path in toml_paths {
        let raw_path = toml_path?.path().to_str().unwrap().to_string();
        if raw_path.contains(".toml") {
            return_paths.push(raw_path);
        }
    }
    for path in return_paths {
        match read_config_toml(path) {
            Ok(config) => configs.push(config),
            _ => {}
        }
    }
    Ok(configs)
}

fn read_config_toml(file_path: String) -> Result<Config, toml::de::Error> {
    let mut file = File::open(file_path).expect("error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Error while reading file to string");
    let config: Config = toml::from_str(&buf)?;
    Ok(config)
}
