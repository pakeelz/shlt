use std::{env::home_dir, fs::File, io::Write, path::PathBuf};

use crate::error::AppError;

pub async fn setup() {
    // Debug soon
    if let Err(_e) = handle_icon_apps().await {}
    if let Err(_e) = handle_desktop_shortcut() {}
}

async fn handle_icon_apps() -> Result<(), AppError> {
    let home_dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut icon_file = home_dir;
    icon_file.push(".local/share/shlt/shlt.png");

    let icon_url = "https://raw.githubusercontent.com/pakeelz/shlt/main/assets/kaaba.png";

    if !icon_file.exists() {
        println!("Downloading assets");
        download_assets(icon_url, &icon_file).await?;
    } else {
        let buf = std::fs::read(&icon_file)?;
        if !infer::image::is_png(&buf) {
            println!("Repairing assets");
            download_assets(icon_url, &icon_file).await?;
        }
    }

    Ok(())
}

fn handle_desktop_shortcut() -> Result<(), AppError> {
    let home_dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut desktop_file = home_dir;
    desktop_file.push(".local/share/applications/shlt.desktop");

    if !desktop_file.exists() {
        println!("Adding shlt.desktop to {desktop_file:?}");
        let content = r"[Desktop Entry]
Type=Application
Name=Shlt
Comment=Jadwal Sholat Indonesia
Exec=/usr/local/bin/shlt
Icon=/home/pakel/.local/share/shlt/shlt.png
Terminal=true
Categories=Utility;Development;";
        let mut file = File::create(desktop_file)?;
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}

async fn download_assets(url: &str, output_path: &PathBuf) -> Result<(), AppError> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let mut file = File::create(output_path)?;
    file.write_all(&bytes)?;
    Ok(())
}

#[tokio::test]
async fn test_handle_icons() {
    handle_icon_apps().await.unwrap();
    handle_desktop_shortcut().unwrap();
}
