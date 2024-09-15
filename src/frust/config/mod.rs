use serde::Deserialize;
use std::fs::{self, File};
use std::io::Read;

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
/// # Arguments
///
/// * `config_directory` - Path to frust project config files.
///
/// # Returns
///
/// Returns a vector [`Vec<Config>`] with all frust project configurations
///
pub fn get_configs(config_directory: &'static str) -> Vec<Config> {
    let toml_paths = fs::read_dir(config_directory).unwrap();
    let mut return_paths: Vec<String> = vec![];
    let mut configs: Vec<Config> = vec![];

    for toml_path in toml_paths {
        let raw_path = toml_path.unwrap().path().to_str().unwrap().to_string();
        if raw_path.contains(".toml") {
            return_paths.push(raw_path);
        }
    }
    for path in return_paths {
        configs.push(read_config_toml(path));
    }
    configs
}

/// Parses a frust project toml and parses it to a [`Config`]
///
/// # Arguments
///
/// * `file_path` - Path to a frust project toml file.
///
/// # Returns
///
/// Returns a [`Config`] parsed from given toml file.
///
fn read_config_toml(file_path: String) -> Config {
    let mut file = File::open(file_path).expect("error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Error while reading file to string");
    let config: Config = toml::from_str(&buf).unwrap();
    config
}
