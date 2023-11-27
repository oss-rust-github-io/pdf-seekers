//! # File Operations module
//! 
//! - Defines the supporting functions for performing file I/O operations
//! - Defines the supporting functions for getting all files in a directory

use std::io::Write;
use lopdf::Document as lopdoc;
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

    // Get last element from split vector
    let last_item: &str = split_dir[split_dir.len()-1];

    // Split last element on "." delimiter
    let split_item: Vec<&str> = last_item.split('.').collect::<Vec<&str>>();

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
                Ok(s) => format!("{}/cache", s.display().to_string()),
                Err(e) => return Err(FileOperationsError::CurrentWorkingDirectoryReadError(e))
            }
        }
    };

    Ok(cache_dir)
}

/// Create given directory if it does not exist
/// 
/// ## Input Parameters
/// `dir_path` defines the directory to be created
/// 
/// ## Returns
/// - None
pub fn create_dir_if_not_exists(dir_path: &String) -> Result<(), FileOperationsError> {
    match std::fs::create_dir_all(dir_path) {
        Ok(_) => {},
        Err(e) => return Err(FileOperationsError::DirectoryCreateError(dir_path.clone(), e))
    };

    Ok(())
}

/// Write messages to provided files
/// 
/// ## Input Parameters
/// `file_path` defines the directory path and file name for the file
/// `data` contains the data to be written to file
/// 
/// ## Returns
/// - None
pub fn write_to_file(file_path: &String, data: String) -> Result<(), FileOperationsError> {
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path) {
            Ok(s) => s,
            Err(e) => return Err(FileOperationsError::FileOpenError(file_path.clone(), e))
        };

    match file.write_all(data.as_bytes()) {
        Ok(_) => {},
        Err(e) => return Err(FileOperationsError::FileWriteError(file_path.clone(), e))
    };

    let data = "\n";
    match file.write_all(data.as_bytes()) {
        Ok(_) => {},
        Err(e) => return Err(FileOperationsError::FileWriteError(file_path.clone(), e))
    };

    Ok(())
}

/// Display log message to terminal
/// 
/// ## Input Parameters
/// `log_flag` defines the flag to indicate if log messages are to be shown on screen
/// `log_msg` contains the message to be displayed
/// 
/// ## Returns
/// - None
pub fn print_log_to_screen(log_flag: &Option<bool>, log_msg: &String) {
    match log_flag {
        Some(s) => match s {
            true => println!("{}", log_msg),
            false => {}
        },
        None => {}
    }
}