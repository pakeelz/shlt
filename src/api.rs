use reqwest::Client;

use crate::{
    error::AppError,
    models::{KabKota, Shalat},
    req::{KabKotaRequest, ShalatRequest},
};

pub async fn get_jadwal_sholat(provinsi: &str, kabkota: &str) -> Result<Shalat, AppError> {
    let client = Client::new();
    let url = "https://equran.id/api/v2/shalat";
    let header_body = ShalatRequest::new(provinsi.to_string(), kabkota.to_string());

    let response = client
        .post(url)
        .json(&header_body)
        .send()
        .await?
        .json::<Shalat>()
        .await?;

    return Ok(response);
}

pub async fn get_possible_kabkota(provinsi: &str) -> Result<KabKota, AppError> {
    let client = Client::new();
    let url = "https://equran.id/api/v2/shalat/kabkota";
    let header_body = KabKotaRequest::from(provinsi.to_string());

    let response = client
        .post(url)
        .json(&header_body)
        .send()
        .await?
        .json::<KabKota>()
        .await?;

    Ok(response)
}
