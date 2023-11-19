use std::fmt::Display;
use chrono::prelude::Utc;

#[derive(Debug)]
pub enum FileOperationsError {
    FileOpenError(String, std::io::Error),
    FileReadError(String, pdf_extract::OutputError),
    PDFFileReadError(String, lopdf::Error),
    PDFFileTextExtractionError(String, u32, lopdf::Error),
    DirectoryReadError(String, std::io::Error),
}

impl Display for FileOperationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileOperationsError::FileOpenError(file_name, err) => 
                write!(f, "[{}] [ERROR] FO0001 - {}: {}", Utc::now(), file_name, err),
            FileOperationsError::FileReadError(file_name, err) => 
                write!(f, "[{}] [ERROR] FO0002 - {}: {}", Utc::now(), file_name, err),
            FileOperationsError::PDFFileReadError(file_name, err) => 
                write!(f, "[{}] [ERROR] FO0003 - {}: {}", Utc::now(), file_name, err),
            FileOperationsError::PDFFileTextExtractionError(file_name, page_num, err) => 
                write!(f, "[{}] [ERROR] FO0004 - {}: Page-{} {}", Utc::now(), file_name, page_num, err),
            FileOperationsError::DirectoryReadError(dir_path, err) => 
                write!(f, "[{}] [ERROR] FO0005 - {}: {}", Utc::now(), dir_path, err),
        }
    }
}

#[derive(Debug)]
pub enum IndexingError {
    IndexDirectoryOpenError(String, tantivy::error::TantivyError),
    IndexDirectoryReadError(String, std::io::Error),
    IndexDirectoryCreateError(String, std::io::Error),
    IndexCreateError(String, tantivy::error::TantivyError),
    IndexWriterCreateError(tantivy::error::TantivyError),
    IndexFieldNotFound(String, tantivy::error::TantivyError),
    IndexDocumentAddError(tantivy::error::TantivyError),
    IndexDocumentCommitError(tantivy::error::TantivyError),
}

impl Display for IndexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexingError::IndexDirectoryOpenError(dir_path, err) => 
                write!(f, "[{}] [ERROR] IE0001 - {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexDirectoryReadError(dir_path, err) => 
                write!(f, "[{}] [ERROR] IE0002 - {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexDirectoryCreateError(dir_path, err) => 
                write!(f, "[{}] [ERROR] IE0003 - {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexCreateError(dir_path, err) => 
                write!(f, "[{}] [ERROR] IE0004 - {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexWriterCreateError(err) => 
                write!(f, "[{}] [ERROR] IE0005 - {}", Utc::now(), err),
            IndexingError::IndexFieldNotFound(field_name, err) => 
                write!(f, "[{}] [ERROR] IE0006 - {}: {}", Utc::now(), field_name, err),
            IndexingError::IndexDocumentAddError(err) => 
                write!(f, "[{}] [ERROR] IE0007 - {}", Utc::now(), err),
            IndexingError::IndexDocumentCommitError(err) => 
                write!(f, "[{}] [ERROR] IE0008 - {}", Utc::now(), err),
        }
    }
}

#[derive(Debug)]
pub enum SearchingError {
    IndexReaderCreateError(tantivy::error::TantivyError),
    IndexFieldNotFound(String, tantivy::error::TantivyError),
    QueryParserError(tantivy::query::QueryParserError),
    KeywordSearchError(tantivy::error::TantivyError),
    SearcherDocumentFetchError(tantivy::error::TantivyError),
}

impl Display for SearchingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchingError::IndexReaderCreateError(err) => 
                write!(f, "[{}] [ERROR] SE0001 - {}", Utc::now(), err),
            SearchingError::IndexFieldNotFound(field_name, err) => 
                write!(f, "[{}] [ERROR] SE0002 - {} - {}", Utc::now(), field_name, err),
            SearchingError::QueryParserError(err) => 
                write!(f, "[{}] [ERROR] SE0003 - {}", Utc::now(), err),
            SearchingError::KeywordSearchError(err) => 
                write!(f, "[{}] [ERROR] SE0004 - {}", Utc::now(), err),
            SearchingError::SearcherDocumentFetchError(err) => 
                write!(f, "[{}] [ERROR] SE0005 - {}", Utc::now(), err),
        }
    }
}

impl std::error::Error for FileOperationsError {}
impl std::error::Error for IndexingError {}
impl std::error::Error for SearchingError {}