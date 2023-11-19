use std::fmt::Display;
use chrono::prelude::Utc;

#[derive(Debug)]
pub enum FileOperationsError {
    FileOpenError(String, std::io::Error),
    FileReadError(String, pdf_extract::OutputError),
}

impl Display for FileOperationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileOperationsError::FileOpenError(file_name, err) => write!(f, "[{}] [ERROR] E00001 - {}: {}", Utc::now(), file_name, err),
            FileOperationsError::FileReadError(file_name, err) => write!(f, "[{}] [ERROR] E00002 - {}: {}", Utc::now(), file_name, err),
        }
    }
}

#[derive(Debug)]
pub enum IndexingError {
    IndexDirectoryOpenError(String, tantivy::error::TantivyError),
    IndexDirectoryReadError(String, std::io::Error),
    IndexDirectoryCreateError(String, std::io::Error),
    IndexCreateError(String, tantivy::error::TantivyError),
    IndexWriterCreateError(tantivy::error::TantivyError)
}

impl Display for IndexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexingError::IndexDirectoryOpenError(dir_path, err) => write!(f, "[{}] [ERROR] E00003 - {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexDirectoryReadError(dir_path, err) => write!(f, "[{}] [ERROR] E00004 - {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexDirectoryCreateError(dir_path, err) => write!(f, "[{}] [ERROR] E00005 - {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexCreateError(dir_path, err) => write!(f, "[{}] [ERROR] E00006 - {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexWriterCreateError(err) => write!(f, "[{}] [ERROR] E00007 - {}", Utc::now(), err),
        }
    }
}

impl std::error::Error for FileOperationsError {}
impl std::error::Error for IndexingError {}
