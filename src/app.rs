use clap::Parser;

use crate::{
    animate,
    api::{get_jadwal_sholat, get_possible_kabkota},
    cli::Args,
    error::AppError,
    models::PROVINSI,
    storage::{read_config, write_config},
    view::{self, select_kabkota, select_provinsi},
};

pub async fn run() -> Result<(), AppError> {
    let cli = Args::parse();

    if let Some(provinsi) = cli.get_provinsi() {
        if let Some(kabkota) = cli.get_kabkota() {
            let res = animate::with_spinner(
                "Mengambil jadwal sholat",
                get_jadwal_sholat(&provinsi, &kabkota),
            )
            .await?;

            let jadwal = res.get_jadwal_today();
            view::print_jadwal_table(jadwal);

            if cli.is_default() {
                write_config(provinsi, kabkota)?
            }
        } else {
            run_with_provinsi_data(&provinsi, cli.is_default()).await?;
        }
    } else {
        match read_config() {
            Ok(loc) => {
                if cli.is_new() {
                    let provinsi = select_provinsi(PROVINSI).unwrap();
                    run_with_provinsi_data(&provinsi, cli.is_default()).await?;
                } else {
                    let res = animate::with_spinner(
                        "Tunggu sebentar",
                        get_jadwal_sholat(loc.get_provinsi(), loc.get_kabkota()),
                    )
                    .await;
                    // let res = get_jadwal_sholat(loc.get_provinsi(), loc.get_kabkota()).await;
                    match res {
                        Ok(res) => {
                            let jadwal = res.get_jadwal_today();
                            view::print_jadwal_table(jadwal);
                        }
                        Err(e) => match e {
                            AppError::RegionNotFound => {
                                let provinsi = select_provinsi(PROVINSI).unwrap();
                                run_with_provinsi_data(&provinsi, true).await?;
                            }
                            _ => eprintln!("{e}"),
                        },
                    }
                }
            }
            Err(_) => {
                let provinsi = select_provinsi(PROVINSI).unwrap();
                run_with_provinsi_data(&provinsi, cli.is_default()).await?;
            }
        }
    }

    Ok(())
}

async fn run_with_provinsi_data(provinsi: &str, is_default: bool) -> Result<(), AppError> {
    let kabkota_data = animate::with_spinner("Mengambil data", get_possible_kabkota(provinsi))
        .await?
        .get_data_kota();
    let kabkota = select_kabkota(&kabkota_data)?;

    let jadwal =
        animate::with_spinner("Tunggu sebentar", get_jadwal_sholat(provinsi, kabkota)).await?;
    // let jadwal = get_jadwal_sholat(provinsi, kabkota).await?;

    view::print_jadwal_table(jadwal.get_jadwal_today());

    if is_default {
        write_config(provinsi.to_string(), kabkota.to_string())?;
    }
    Ok(())
}
