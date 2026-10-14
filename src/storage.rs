use std::{env::home_dir, fs, path::PathBuf};

use crate::{api::Schedule, error::AppError};

pub fn write_config(data: &Schedule) -> Result<(), AppError> {
    let mut config_dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push(".config/shlt");

    fs::create_dir_all(&config_dir)?;

    let mut config_file = config_dir;
    config_file.push("shlt.json");

    let config_content = serde_json::to_string_pretty(data)?;

    fs::write(config_file, config_content)?;

    Ok(())
}

pub fn need_update() -> Result<bool, AppError> {
    let mut config_file = home_dir().unwrap_or_else(|| PathBuf::from("."));
    config_file.push(".config/shlt/shlt.json");

    let config_content = fs::read_to_string(config_file)?;
    let content = serde_json::from_str::<Schedule>(&config_content)?;

    let tanggal_lengkap: &Vec<&str> = &content.data[0].tanggal_lengkap.splitn(3, '-').collect();

    let month = tanggal_lengkap[1]
        .parse::<u8>()
        .map_err(|f| AppError::Other(f.to_string()))?;

    if month == month_now()? {
        return Ok(false);
    }

    Ok(true)
}

pub fn read_config() -> Result<Schedule, AppError> {
    let mut config_file = home_dir().unwrap_or_else(|| PathBuf::from("."));
    config_file.push(".config/shlt/shlt.json");

    let config_content = fs::read_to_string(config_file)?;
    let shalat = serde_json::from_str::<Schedule>(&config_content)?;

    Ok(shalat)
}

fn month_now() -> Result<u8, AppError> {
    let month = chrono::Local::now().format("%m").to_string();
    let month = month
        .parse::<u8>()
        .map_err(|f| AppError::Other(f.to_string()))?;
    Ok(month)
}

// #[tokio::test]
// async fn test_write() {
//     let shalat = get_jadwal_sholat("Jawa Timur", "Kab. Pasuruan")
//         .await
//         .unwrap();
//     write_config(&shalat).unwrap();
// }

#[test]
fn test_check_config() {
    let _test = need_update().unwrap();
    println!("{}", _test);
}

#[test]
fn test_read() {
    let shalat = read_config().unwrap();
    println!("{:?}", shalat);
}
