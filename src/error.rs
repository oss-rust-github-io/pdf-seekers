//! # Error module
//! 
//! - Defines the error types for file I/O operations
//! - Defines the error types for index operations
//! - Defines the error types for search operations

use std::fmt::Display;


#[derive(Debug)]
/// Defines the file I/O error types
pub enum FileOperationsError {
    /// Unable to open PDF file
    PDFFileReadError(String, lopdf::Error),
    /// Unable to read contents from PDF file
    PDFFileTextExtractionError(String, u32, lopdf::Error),
    /// Unable to read contents from directory
    DirectoryReadError(String, std::io::Error),
    /// Unable to read the current working directory
    CurrentWorkingDirectoryReadError(std::io::Error),
    /// Unable to create directory
    DirectoryCreateError(String, std::io::Error),
    /// Unable to open file
    FileOpenError(String, std::io::Error),
    /// Unable to write to file
    FileWriteError(String, std::io::Error)
}

impl Display for FileOperationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileOperationsError::PDFFileReadError(file_name, err) => 
                write!(f, "[FO0001_PDFFileReadError] {}: {}", file_name, err),
            FileOperationsError::PDFFileTextExtractionError(file_name, page_num, err) => 
                write!(f, "[FO0002_PDFFileTextExtractionError] {}: Page-{} {}", file_name, page_num, err),
            FileOperationsError::DirectoryReadError(dir_path, err) => 
                write!(f, "[FO0003_DirectoryReadError] {}: {}", dir_path, err),
            FileOperationsError::CurrentWorkingDirectoryReadError(err) => 
                write!(f, "[FO0004_CurrentWorkingDirectoryReadError] {}", err),
            FileOperationsError::DirectoryCreateError(dir_path, err) => 
                write!(f, "[FO0005_DirectoryCreateError] {}: {}", dir_path, err),
            FileOperationsError::FileOpenError(log_file, err) => 
                write!(f, "[FO0006_FileOpenError] {}: {}", log_file, err),
            FileOperationsError::FileWriteError(log_file, err) => 
                write!(f, "[FO0007_FileWriteError] {}: {}", log_file, err),
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
                write!(f, "[IE0001_IndexDirectoryOpenError] {}: {}", dir_path, err),
            IndexingError::IndexDirectoryReadError(dir_path, err) => 
                write!(f, "[IE0002_IndexDirectoryReadError] {}: {}", dir_path, err),
            IndexingError::IndexDirectoryCreateError(dir_path, err) => 
                write!(f, "[IE0003_IndexDirectoryCreateError] {}: {}", dir_path, err),
            IndexingError::IndexCreateError(dir_path, err) => 
                write!(f, "[IE0004_IndexCreateError] {}: {}", dir_path, err),
            IndexingError::IndexWriterCreateError(err) => 
                write!(f, "[IE0005_IndexWriterCreateError] {}", err),
            IndexingError::IndexFieldNotFound(field_name, err) => 
                write!(f, "[IE0006_IndexFieldNotFound] {}: {}", field_name, err),
            IndexingError::IndexDocumentAddError(err) => 
                write!(f, "[IE0007_IndexDocumentAddError] {}", err),
            IndexingError::IndexDocumentCommitError(err) => 
                write!(f, "[IE0008_IndexDocumentCommitError] {}", err),
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
                write!(f, "[SE0001_IndexReaderCreateError] {}", err),
            SearchingError::IndexFieldNotFound(field_name, err) => 
                write!(f, "[SE0002_IndexFieldNotFound] {} - {}", field_name, err),
            SearchingError::QueryParserError(err) => 
                write!(f, "[SE0003_QueryParserError] {}", err),
            SearchingError::KeywordSearchError(err) => 
                write!(f, "[SE0004_KeywordSearchError] {}", err),
            SearchingError::SearcherDocumentFetchError(err) => 
                write!(f, "[SE0005_SearcherDocumentFetchError] {}", err),
        }
    }
}

impl std::error::Error for FileOperationsError {}
impl std::error::Error for IndexingError {}
impl std::error::Error for SearchingError {}