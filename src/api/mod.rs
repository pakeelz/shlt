use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::api::req::KabKotaRequest;
use crate::api::req::ShalatRequest;
use crate::error::AppError;

mod req;

pub const PROVINSI: [&str; 34] = [
    "Aceh",
    "Bali",
    "Banten",
    "Bengkulu",
    "D.I. Yogyakarta",
    "DKI Jakarta",
    "Gorontalo",
    "Jambi",
    "Jawa Barat",
    "Jawa Tengah",
    "Jawa Timur",
    "Kalimantan Barat",
    "Kalimantan Selatan",
    "Kalimantan Tengah",
    "Kalimantan Timur",
    "Kalimantan Utara",
    "Kepulauan Bangka Belitung",
    "Kepulauan Riau",
    "Lampung",
    "Maluku",
    "Maluku Utara",
    "Nusa Tenggara Barat",
    "Nusa Tenggara Timur",
    "Papua",
    "Papua Barat",
    "Riau",
    "Sulawesi Barat",
    "Sulawesi Selatan",
    "Sulawesi Tengah",
    "Sulawesi Tenggara",
    "Sulawesi Utara",
    "Sumatera Barat",
    "Sumatera Selatan",
    "Sumatera Utara",
];

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Shalat {
    pub data: Data,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Data {
    pub kabkota: String,
    pub jadwal: Vec<Jadwal>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Jadwal {
    tanggal: u8,
    pub tanggal_lengkap: String,
    pub subuh: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct KabKota {
    pub data: Vec<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Schedule {
    pub province: String,
    pub kabkota: String,
    pub data: Vec<Jadwal>,
}

impl Schedule {
    fn from_shalat(province: &str, shalat: Shalat) -> Self {
        Self {
            province: province.to_string(),
            kabkota: shalat.data.kabkota,
            data: shalat.data.jadwal,
        }
    }
}

pub async fn get_jadwal_sholat(provinsi: &str, kabkota: &str) -> Result<Schedule, AppError> {
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

    let response = Schedule::from_shalat(provinsi, response);

    Ok(response)
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

#[tokio::test]
async fn test_jadwal() {
    let jadwal = get_jadwal_sholat("Jawa Timur", "Kab. Pasuruan")
        .await
        .unwrap();
    println!("{jadwal:?}");
}
