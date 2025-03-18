use std::fmt;
use std::io;

/// CSV解析に関連するエラー
#[derive(Debug)]
pub enum CsvError {
    /// IOエラー
    Io(io::Error),
    /// エンコーディングエラー
    Encoding(String),
    /// CSVフォーマットエラー
    Format(String),
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvError::Io(err) => write!(f, "IO error: {}", err),
            CsvError::Encoding(msg) => write!(f, "Encoding error: {}", msg),
            CsvError::Format(msg) => write!(f, "CSV format error: {}", msg),
        }
    }
}

impl std::error::Error for CsvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CsvError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for CsvError {
    fn from(err: io::Error) -> Self {
        CsvError::Io(err)
    }
}
