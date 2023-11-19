//! # File Operations module
//! 
//! - Defines the supporting functions for performing file I/O operations
//! - Defines the supporting functions for getting all files in a directory

use crate::error::FileOperationsError;

/// Read a PDF files to extract its contents
/// 
/// ## Input Parameters
/// - `pdf_file` contains the PDF file to be read
/// 
/// ## Returns
/// - Extract text from PDF file
pub fn read_pdf(pdf_file: &str) -> Result<String, FileOperationsError> {
    // Open the PDF file
    let file: Vec<u8>= match std::fs::read(pdf_file) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::FileOpenError(pdf_file.to_string(), e))
    };
    
    // Extract text from the PDF
    let text: String = match pdf_extract::extract_text_from_mem(&file) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::FileReadError(pdf_file.to_string(), e))
    };

    Ok(text)
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