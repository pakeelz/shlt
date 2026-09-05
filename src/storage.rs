use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    location: Location,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    provinsi: String,
    kabkota: String,
}

impl Config {
    fn from(provinsi: String, kabkota: String) -> Self {
        let location = Location::from(provinsi, kabkota);
        Self { location }
    }

    fn get_location(self) -> Location {
        self.location
    }
}

impl Location {
    fn from(provinsi: String, kabkota: String) -> Self {
        Self { provinsi, kabkota }
    }

    pub fn get_provinsi(&self) -> &str {
        &self.provinsi
    }

    pub fn get_kabkota(&self) -> &str {
        &self.kabkota
    }
}

pub fn write_config(provinsi: String, kabkota: String) -> Result<(), AppError> {
    let home_dir = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));

    let cnfg = Config::from(provinsi, kabkota);

    let mut config_dir = home_dir;
    config_dir.push(".config");
    config_dir.push("shlt");

    fs::create_dir_all(&config_dir)?;

    let mut config_file = config_dir;
    config_file.push("config.toml");

    let config_content = toml::to_string(&cnfg).unwrap();
    fs::write(config_file, config_content)?;
    Ok(())
}

pub fn read_config() -> Result<Location, AppError> {
    let home_dir = env::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut config_file = home_dir;
    config_file.push(".config");
    config_file.push("shlt");
    config_file.push("config.toml");

    let config_content = fs::read_to_string(config_file)?;
    let content = toml::from_str::<Config>(&config_content)?;
    Ok(content.get_location())
}
