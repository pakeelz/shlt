use serde::Serialize;

#[derive(Serialize)]
pub struct ShalatRequest {
    provinsi: String,
    kabkota: String,
}

#[derive(Serialize)]
pub struct KabKotaRequest {
    provinsi: String,
}

impl ShalatRequest {
    pub fn new(provinsi: String, kabkota: String) -> Self {
        Self { provinsi, kabkota }
    }
}

impl KabKotaRequest {
    pub fn from(provinsi: String) -> Self {
        Self { provinsi }
    }
}
