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

pub trait View {
    fn get_kabkota(&self) -> &str;
    fn get_subuh(&self) -> &str;
    fn get_dzuhur(&self) -> &str;
    fn get_ashar(&self) -> &str;
    fn get_maghrib(&self) -> &str;
    fn get_isya(&self) -> &str;
}

#[derive(Deserialize, Debug)]
pub struct Shalat {
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub kabkota: String,
    pub jadwal: Vec<Jadwal>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Jadwal {
    tanggal: u8,
    pub tanggal_lengkap: String,
    pub subuh: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String,
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
    pub fn get_jadwal_today(self) -> JadwalShalat {
        let now = chrono::Local::now()
            .format("%d")
            .to_string()
            .parse::<usize>()
            .unwrap();

        let kabkota = self.data.kabkota;
        let jadwal = self.data.jadwal[now - 1].clone();
        JadwalShalat::from_jadwal_and_kabkota(jadwal, kabkota)
    }
}

impl View for JadwalShalat {
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
