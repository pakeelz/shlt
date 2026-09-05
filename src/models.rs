use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub struct Shalat {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    kabkota: String,
    jadwal: Vec<Jadwal>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Jadwal {
    tanggal: u8,
    tanggal_lengkap: String,
    subuh: String,
    dzuhur: String,
    ashar: String,
    maghrib: String,
    isya: String,
}

#[allow(dead_code)]
pub struct JadwalShalat {
    pub kabkota: String,
    pub subuh: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct KabKota {
    data: Vec<String>,
}

impl Shalat {
    pub fn get_jadwal_today(&self) -> JadwalShalat {
        let now = chrono::Local::now()
            .format("%d")
            .to_string()
            .parse::<usize>()
            .unwrap();

        let kabkota = self.data.kabkota.clone();
        let jadwal = self.data.jadwal[now - 1].clone();
        JadwalShalat::from_jadwal_and_kabkota(jadwal, kabkota)
    }
}

impl JadwalShalat {
    pub fn from_jadwal_and_kabkota(jadwal: Jadwal, kabkota: String) -> Self {
        Self {
            kabkota,
            subuh: jadwal.subuh,
            dzuhur: jadwal.dzuhur,
            ashar: jadwal.ashar,
            maghrib: jadwal.maghrib,
            isya: jadwal.isya,
        }
    }
}

impl KabKota {
    pub fn get_data_kota(self) -> Vec<String> {
        self.data
    }
}
