use std::{
    error::Error,
    fmt::{self},
};

#[derive(Debug)]
pub enum AppError {
    RegionNotFound,
    ConnectionError,
    ValidateInputProvinsiError,
    IoError(String),
    ConfigError,
    ConfigNotFound,
    UnknownError(String),
}

impl Error for AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::RegionNotFound => {
                write!(f, "Harap masukkan Provinsi dan Kabupaten / Kota yang valid")
            }
            AppError::ConnectionError => write!(f, "Koneksi Error"),
            AppError::ValidateInputProvinsiError => write!(f, "Harap masukkan nomor yang valid"),
            AppError::IoError(e) => write!(f, "{e}"),
            AppError::ConfigError => {
                write!(f, "Config error")
            }
            AppError::ConfigNotFound => write!(f, "Config tidak ditemukan"),
            AppError::UnknownError(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_decode() {
            Self::RegionNotFound
        } else if err.is_connect() {
            Self::ConnectionError
        } else {
            Self::UnknownError(err.to_string())
        }
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(_err: std::num::ParseIntError) -> Self {
        Self::ValidateInputProvinsiError
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::ConfigNotFound,
            _ => Self::IoError(err.to_string()),
        }
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(_err: toml::ser::Error) -> Self {
        Self::ConfigError
    }
}

impl From<toml::de::Error> for AppError {
    fn from(_err: toml::de::Error) -> Self {
        Self::ConfigError
    }
}

impl From<dialoguer::Error> for AppError {
    fn from(err: dialoguer::Error) -> Self {
        Self::UnknownError(err.to_string())
    }
}
