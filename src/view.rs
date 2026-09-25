use comfy_table::{
    Cell, CellAlignment,
    Color::{self},
    Table, presets,
};
use dialoguer::{FuzzySelect, theme::ColorfulTheme};

use crate::{error::AppError, models::View};

pub fn print_jadwal_table<T>(jadwal: T)
where
    T: View,
{
    let mut table = Table::new();
    table
        .load_style(presets::ASCII_MARKDOWN)
        .set_header(vec![
            Cell::new(jadwal.get_kabkota())
                .set_alignment(CellAlignment::Center)
                .fg(Color::DarkYellow),
        ])
        .add_row(vec!["Subuh", jadwal.get_subuh()])
        .add_row(vec!["Dzuhur", jadwal.get_dzuhur()])
        .add_row(vec!["Ashar", jadwal.get_ashar()])
        .add_row(vec!["Maghrib", jadwal.get_maghrib()])
        .add_row(vec!["Isya", jadwal.get_isya()]);
    println!("\n{}\n", table);
}

pub fn print_raw_jadwal<T>(jadwal: T)
where
    T: View,
{
    println!("{}", jadwal.get_kabkota());
    println!("{}", jadwal.get_subuh());
    println!("{}", jadwal.get_dzuhur());
    println!("{}", jadwal.get_ashar());
    println!("{}", jadwal.get_maghrib());
    println!("{}", jadwal.get_isya());
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
