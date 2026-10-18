use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum AppError {
    ConnectionError,
    TuiError(String),
    ConfigNotFound,
    WriteConfigError(String),
    TerminalToSmall,
    Other(String),
}

impl Error for AppError {}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ConnectionError => write!(f, "No Internet Connection"),
            AppError::TuiError(e) => write!(f, "TUI Error: {e}"),
            AppError::ConfigNotFound => write!(f, "Config Not Found"),
            AppError::WriteConfigError(e) => write!(f, "Write Config Error: {e}"),
            AppError::TerminalToSmall => write!(f, "Terminal to small"),
            AppError::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect() {
            Self::ConnectionError
        } else {
            Self::Other(err.to_string())
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::ConfigNotFound,
            _ => Self::WriteConfigError(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::WriteConfigError(err.to_string())
    }
}
