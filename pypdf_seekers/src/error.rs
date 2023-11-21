//! # Error module
//! 
//! - Defines the error types for file I/O operations
//! - Defines the error types for index operations
//! - Defines the error types for search operations

use std::fmt::Display;
use chrono::prelude::Utc;

#[derive(Debug)]
/// Defines the file I/O error types
pub enum FileOperationsError {
    /// Unable to open a file
    FileOpenError(String, std::io::Error),
    /// Unable to read contents from a file
    FileReadError(String, pdf_extract::OutputError),
    /// Unable to open PDF file
    PDFFileReadError(String, lopdf::Error),
    /// Unable to read contents from PDF file
    PDFFileTextExtractionError(String, u32, lopdf::Error),
    /// Unable to read contents from directory
    DirectoryReadError(String, std::io::Error),
}

impl Display for FileOperationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileOperationsError::FileOpenError(file_name, err) => 
                write!(f, "[{}] [ERROR] [FO0001_FileOpenError] {}: {}", Utc::now(), file_name, err),
            FileOperationsError::FileReadError(file_name, err) => 
                write!(f, "[{}] [ERROR] [FO0002_FileReadError] {}: {}", Utc::now(), file_name, err),
            FileOperationsError::PDFFileReadError(file_name, err) => 
                write!(f, "[{}] [ERROR] [FO0003_PDFFileReadError] {}: {}", Utc::now(), file_name, err),
            FileOperationsError::PDFFileTextExtractionError(file_name, page_num, err) => 
                write!(f, "[{}] [ERROR] [FO0004_PDFFileTextExtractionError] {}: Page-{} {}", Utc::now(), file_name, page_num, err),
            FileOperationsError::DirectoryReadError(dir_path, err) => 
                write!(f, "[{}] [ERROR] [FO0005_DirectoryReadError] {}: {}", Utc::now(), dir_path, err),
        }
    }
}

#[derive(Debug)]
/// Defines the indexing operation error types
pub enum IndexingError {
    /// Unable to open index directory
    IndexDirectoryOpenError(String, tantivy::error::TantivyError),
    /// Unable to read contents from index directory
    IndexDirectoryReadError(String, std::io::Error),
    /// Unable to create new index directory
    IndexDirectoryCreateError(String, std::io::Error),
    /// Unable to create Tantivy index object
    IndexCreateError(String, tantivy::error::TantivyError),
    /// Unable to create Tantivy index writer
    IndexWriterCreateError(tantivy::error::TantivyError),
    /// Unable to find field in indexed files for performing search
    IndexFieldNotFound(String, tantivy::error::TantivyError),
    /// Unable to add new index file to index directory
    IndexDocumentAddError(tantivy::error::TantivyError),
    /// Unable to commit new index file to index directory
    IndexDocumentCommitError(tantivy::error::TantivyError),
}

impl Display for IndexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexingError::IndexDirectoryOpenError(dir_path, err) => 
                write!(f, "[{}] [ERROR] [IE0001_IndexDirectoryOpenError] {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexDirectoryReadError(dir_path, err) => 
                write!(f, "[{}] [ERROR] [IE0002_IndexDirectoryReadError] {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexDirectoryCreateError(dir_path, err) => 
                write!(f, "[{}] [ERROR] [IE0003_IndexDirectoryCreateError] {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexCreateError(dir_path, err) => 
                write!(f, "[{}] [ERROR] [IE0004_IndexCreateError] {}: {}", Utc::now(), dir_path, err),
            IndexingError::IndexWriterCreateError(err) => 
                write!(f, "[{}] [ERROR] [IE0005_IndexWriterCreateError] {}", Utc::now(), err),
            IndexingError::IndexFieldNotFound(field_name, err) => 
                write!(f, "[{}] [ERROR] [IE0006_IndexFieldNotFound] {}: {}", Utc::now(), field_name, err),
            IndexingError::IndexDocumentAddError(err) => 
                write!(f, "[{}] [ERROR] [IE0007_IndexDocumentAddError] {}", Utc::now(), err),
            IndexingError::IndexDocumentCommitError(err) => 
                write!(f, "[{}] [ERROR] [IE0008_IndexDocumentCommitError] {}", Utc::now(), err),
        }
    }
}

#[derive(Debug)]
/// Defines the search operation error types
pub enum SearchingError {
    /// Unable to create Tantivy index reader
    IndexReaderCreateError(tantivy::error::TantivyError),
    /// Unable to find field in indexed files for performing search
    IndexFieldNotFound(String, tantivy::error::TantivyError),
    /// Unable to create Tantivy query parser object
    QueryParserError(tantivy::query::QueryParserError),
    /// Unable to run keyword search on extracted PDF contents
    KeywordSearchError(tantivy::error::TantivyError),
    /// Unable to get matched PDF files from Tantivy index
    SearcherDocumentFetchError(tantivy::error::TantivyError),
}

impl Display for SearchingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchingError::IndexReaderCreateError(err) => 
                write!(f, "[{}] [ERROR] [SE0001_IndexReaderCreateError] {}", Utc::now(), err),
            SearchingError::IndexFieldNotFound(field_name, err) => 
                write!(f, "[{}] [ERROR] [SE0002_IndexFieldNotFound] {} - {}", Utc::now(), field_name, err),
            SearchingError::QueryParserError(err) => 
                write!(f, "[{}] [ERROR] [SE0003_QueryParserError] {}", Utc::now(), err),
            SearchingError::KeywordSearchError(err) => 
                write!(f, "[{}] [ERROR] [SE0004_KeywordSearchError] {}", Utc::now(), err),
            SearchingError::SearcherDocumentFetchError(err) => 
                write!(f, "[{}] [ERROR] [SE0005_SearcherDocumentFetchError] {}", Utc::now(), err),
        }
    }
}

impl std::error::Error for FileOperationsError {}
impl std::error::Error for IndexingError {}
impl std::error::Error for SearchingError {}