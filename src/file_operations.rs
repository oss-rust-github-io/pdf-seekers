//! # File Operations module
//! 
//! - Defines the supporting functions for performing file I/O operations
//! - Defines the supporting functions for getting all files in a directory

use lopdf::Document as lopdoc;
use log::trace;
use crate::error::FileOperationsError;

/// Read a PDF files to extract its contents
/// 
/// ## Input Parameters
/// - `pdf_file` contains the PDF file to be read
/// 
/// ## Returns
/// - Extract text from PDF file
pub fn read_pdf(pdf_file: &str) -> Result<(Vec<u32>, Vec<String>), FileOperationsError> {
    // Read the PDF file
    let doc = match lopdoc::load(&pdf_file) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::PDFFileReadError(pdf_file.to_string().clone(), e))
    };
    trace!(target:"other_logging", "PDF document `{}` with {} pages read successfully.", &pdf_file, &doc.get_pages().len());

    // Get all pages in the PDF file
    let pages = doc.get_pages();
    let mut pdf_page_nums: Vec<u32> = Vec::new();
    let mut pdf_texts: Vec<String> = Vec::new();

    // Traverse through the PDF pages to extract text
    for (i, _) in pages.iter().enumerate() {
        let page_number: u32 = (i + 1) as u32;
        
        // Extract text from a single PDF page
        let text: String = match doc.extract_text(&[page_number]) {
            Ok(s) => s,
            Err(e) => return Err(FileOperationsError::PDFFileTextExtractionError(pdf_file.to_string().clone(), page_number, e))
        };
        
        pdf_page_nums.push(page_number);
        pdf_texts.push(text);
    }

    Ok((pdf_page_nums, pdf_texts))
}

/// Checks if given input is a file or directory
/// 
/// ## Input Parameters
/// - `file_or_directory` defines the input path to be analysed
/// 
/// ## Returns
/// - True if input path is a directory, else False
pub fn check_if_directory(file_or_directory: &str) -> bool {
    // Split given directory path on "/" delimiter
    let split_dir: Vec<&str> = file_or_directory.split('/').collect::<Vec<&str>>();
    trace!(target:"other_logging", "split_dir: {:?}", split_dir);

    // Get last element from split vector
    let last_item: &str = split_dir[split_dir.len()-1];
    trace!(target:"other_logging", "last_item: {}", last_item);

    // Split last element on "." delimiter
    let split_item: Vec<&str> = last_item.split('.').collect::<Vec<&str>>();
    trace!(target:"other_logging", "split_item: {:?}", split_item);

    // If last element doesn't contain any file extension, then it's a directory
    if split_item.len() == 1 {
        return true
    }
    else {
        return false
    }
}

/// Gets all file names in a given directory
/// 
/// ## Input Parameters
/// - `directory` defines the input path to be analysed
/// 
/// ## Returns
/// - Vector of file names extracted from directory
pub fn get_files_in_directory(directory: &str) -> Result<Vec<String>, FileOperationsError> {
    let mut files_list: Vec<String> = Vec::new();

    // Read contents of the directory
    let file_paths = match std::fs::read_dir(directory) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::DirectoryReadError(directory.to_string(), e))
    };

    // Access the files names in directory and push to vector
    for file in file_paths {
        let file_name = file.unwrap().file_name().into_string().unwrap();
        files_list.push(format!("{}/{}",directory, file_name));
    }

    Ok(files_list)
}

/// Return the cache directory if provided, else return the current working directory
/// 
/// ## Input Parameters
/// `cache_path` defines the input path for storing the indexed files, log files, and tracker files
/// 
/// ## Returns
/// - Cache directory or current working directory
pub fn get_cache_dir(cache_path: &Option<String>) -> Result<String, FileOperationsError> {
    let cache_dir: String = match cache_path {
        Some(s) => s.clone(),
        None => {
            match std::env::current_dir() {
                Ok(s) => format!("{}/.cache", s.display().to_string()),
                Err(e) => return Err(FileOperationsError::CurrentWorkingDirectoryReadError(e))
            }
        }
    };

    Ok(cache_dir)
}

/// Create cache directory if it does not exist
/// 
/// ## Input Parameters
/// `cache_path` defines the input path for storing the indexed files, log files, and tracker files
/// 
/// ## Returns
/// - None
pub fn create_cache_dir_if_not_exists(cache_path: &Option<String>) -> Result<String, FileOperationsError> {
    // Get cache directory path
    let cache_dir: String = get_cache_dir(&cache_path)?;

    match std::fs::create_dir_all(&cache_dir) {
        Ok(_) => {},
        Err(e) => return Err(FileOperationsError::DirectoryCreateError(cache_dir, e))
    };

    Ok(cache_dir)
}
