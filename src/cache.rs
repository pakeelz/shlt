#![allow(unused)]

use std::{
    env::{self, home_dir},
    fs,
    path::PathBuf,
};

use clap::error::Result;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    models::{Jadwal, JadwalShalat, Shalat, View},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    kabkota: String,
    tanggal_lengkap: String,
    subuh: String,
    dzuhur: String,
    ashar: String,
    maghrib: String,
    isya: String,
}

impl From<&Shalat> for Cache {
    fn from(shalat: &Shalat) -> Self {
        let now = chrono::Local::now()
            .format("%d")
            .to_string()
            .parse::<usize>()
            .unwrap();

        let kabkota = shalat.data.kabkota.clone();
        let jadwal = shalat.data.jadwal[now - 1].clone();

        Self {
            kabkota: kabkota,
            tanggal_lengkap: jadwal.tanggal_lengkap,
            subuh: jadwal.subuh,
            dzuhur: jadwal.dzuhur,
            ashar: jadwal.ashar,
            maghrib: jadwal.maghrib,
            isya: jadwal.isya,
        }
    }
}

impl View for Cache {
    fn get_kabkota(&self) -> &str {
        &self.kabkota
    }
    fn get_subuh(&self) -> &str {
        &self.subuh
    }
    fn get_dzuhur(&self) -> &str {
        &self.dzuhur
    }
    fn get_ashar(&self) -> &str {
        &self.ashar
    }
    fn get_maghrib(&self) -> &str {
        &self.maghrib
    }
    fn get_isya(&self) -> &str {
        &self.isya
    }
}

pub fn read_cache() -> Result<Cache, AppError> {
    let mut cache_dir = cache_dir();
    cache_dir.push("shlt/shlt.json");

    let content = fs::read_to_string(cache_dir)?;
    let cache = serde_json::from_str::<Cache>(&content)?;
    Ok(cache)
}

// Checking if the data cache is up to date
// If yes return true
pub fn is_update() -> Result<bool, AppError> {
    let mut cache_dir = cache_dir();
    cache_dir.push("shlt/shlt.json");

    let content = fs::read_to_string(cache_dir);

    match content {
        Ok(e) => {
            let cache = serde_json::from_str::<Cache>(&e)?;
            if cache.tanggal_lengkap == utils::take_date_now() {
                return Ok(true);
            }
        }
        Err(_) => return Ok(false),
    }

    Ok(false)
}

pub fn write_cache(shalat: &Shalat) -> Result<(), AppError> {
    let mut cache_dir = cache_dir();

    // Buat struct cache untuk serde_json
    let cfg = Cache::from(shalat);

    // Push menjadi .cache/shlt
    cache_dir.push("shlt");

    fs::create_dir_all(&cache_dir)?;

    // Push menjadi .cache/shlt/shlt.json
    let mut config_file = cache_dir;
    config_file.push("shlt.json");

    let content = serde_json::to_string_pretty(&cfg)?;
    fs::write(config_file, content)?;

    Ok(())
}

fn cache_dir() -> PathBuf {
    let cache_var = env::var("XDG_CACHE_HOME").map(PathBuf::from);
    let mut cache_dir = env::var("HOME")
        .map(PathBuf::from)
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));

    if let Ok(cache) = cache_var {
        return cache;
    }

    cache_dir.push(".cache");
    return cache_dir;
}

mod utils {
    pub fn take_date_now() -> String {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    }
}

#[test]
fn test_check_cache() {
    assert_eq!(is_update().unwrap(), false);
}

#[test]
fn test_write_cache() {}
