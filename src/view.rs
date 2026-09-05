use comfy_table::{
    Cell, CellAlignment,
    Color::{self},
    Table, presets,
};
use dialoguer::{FuzzySelect, theme::ColorfulTheme};

use crate::{error::AppError, models::JadwalShalat};

pub fn print_jadwal_table(jadwal: JadwalShalat) {
    let mut table = Table::new();
    table
        .load_style(presets::ASCII_MARKDOWN)
        .set_header(vec![
            Cell::new(jadwal.kabkota)
                .set_alignment(CellAlignment::Center)
                .fg(Color::DarkYellow),
        ])
        .add_row(vec!["Subuh", &jadwal.subuh])
        .add_row(vec!["Dzuhur", &jadwal.dzuhur])
        .add_row(vec!["Ashar", &jadwal.ashar])
        .add_row(vec!["Maghrib", &jadwal.maghrib])
        .add_row(vec!["Isya", &jadwal.isya]);
    println!("\n{}", table);
}

pub fn select_provinsi(provinsi: [&str; 34]) -> Result<String, AppError> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pilih Provinsi:")
        .default(0)
        .items(&provinsi)
        .interact()?;

    Ok(provinsi[selection].to_string())
}

pub fn select_kabkota(kabkota: &Vec<String>) -> Result<&str, AppError> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pilih Kabupaten / Kota:")
        .default(0)
        .items(kabkota)
        .interact()?;

    Ok(&kabkota[selection])
}
