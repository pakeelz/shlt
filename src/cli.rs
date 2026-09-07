use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Nama Provinsi
    provinsi: Option<Vec<String>>,

    /// Nama Kabupaten atau Kota
    #[arg(short, num_args = 2..)]
    kabkota: Option<Vec<String>>,

    /// Menyimpan kota / kabupaten di dalam config
    #[arg(short)]
    default: bool,

    /// Menampilkan menu interaktif
    #[arg(short)]
    new: bool,

    /// Menampilkan raw data, urutan dari subuh sampai isya
    #[arg(short, long)]
    raw: bool,
}

impl Args {
    pub fn get_provinsi(&self) -> Option<String> {
        argument_format(&self.provinsi)
    }

    pub fn get_kabkota(&self) -> Option<String> {
        argument_format(&self.kabkota)
    }

    pub fn is_default(&self) -> bool {
        self.default
    }

    pub fn is_new(&self) -> bool {
        self.new
    }

    pub fn is_raw(&self) -> bool {
        self.raw
    }
}

fn argument_format(data: &Option<Vec<String>>) -> Option<String> {
    if let Some(data) = data {
        let formatted_data = data.join(" ");
        return Some(formatted_data);
    }
    None
}
