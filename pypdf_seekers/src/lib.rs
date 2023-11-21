//! # PDF Seeker
//! Simple parser and information extractor from PDF documents based on keyword search functionality (powered by Rust)
//!
//! ## Key Features:
//! - Indexing capability on single PDF file or directory containing multiple PDF files
//! - Search for keywords across multiple PDF files to get relevant information
//! - Get number of pages in PDF file, the page numbers containing the search term, and surrounding text aroung the search term
//!
//! ## Getting Started:
//! Visit the [pdf-seeker official repository](https://github.com/oss-rust-github-io/pdf-seekers.git) for more information.

pub mod error;
pub mod file_operations;
pub mod index_operations;
pub mod search_operations;

use file_operations::*;
use index_operations::*;
use search_operations::*;
use chrono::prelude::Utc;
use pyo3::prelude::*;

#[pyfunction]
/// Create indexes for either single PDF file or directory containing multiple PDF files
/// 
/// ## Input Parameters
/// - `file_or_directory` defines the input path for single PDF file or directory containing multiple PDF files
/// - `index_path` defines the input path for storing the indexed files
pub fn indexing_contents(file_or_directory: &str, index_path: &str) -> Result<(), std::io::Error> {
    // Check if indexing to be performed on single file or directory of files
    let dir_flag: bool = check_if_directory(file_or_directory);

    if dir_flag {
        // Get all file names in directory
        let files_list = match get_files_in_directory(file_or_directory) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        
        // Run indexing on all files in directory
        for file in &files_list {
            file_indexing(&file, index_path);
            println!("[{}] [INFO] {} - Indexing completed.", Utc::now(), file);
        }
    }
    else {
        // Run indexing on single file
        file_indexing(file_or_directory, index_path);
        println!("[{}] [INFO] {} - Indexing completed.", Utc::now(), file_or_directory);
    }

    Ok(())
}

#[pyfunction]
/// Search for a keyword in either single PDF file or directory containing multiple PDF files
/// 
/// ## Input Parameters
/// - `file_or_directory` defines the input path for single PDF file or directory containing multiple PDF files
/// - `index_path` defines the input path for storing the indexed files
/// - `search_term` defines the keyword to be searched in PDF documents
pub fn search_term_in_file(file_or_directory: &str, index_path: &str, search_term: String) -> Result<Vec<PDFMetadata>, std::io::Error> {
    // Check if indexing to be performed on single file or directory of files
    let dir_flag: bool = check_if_directory(file_or_directory);
    
    // Create or open the Tantivy index
    let index: tantivy::Index = match create_or_open_index(index_path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Search for a term in the indexed PDFs
    let matched_docs: Vec<String> = match search_keyword(&index, &search_term) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Run analysis on PDF documents containing the search term
    let mut metadata_vec: Vec<PDFMetadata> = Vec::new();
    if dir_flag {
        // Get all file names in directory
        let files_list = match get_files_in_directory(file_or_directory) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        
        // Traverse the matched PDF documents (containing the search term) to display the metadata information
        for doc in matched_docs {
            if files_list.contains(&doc) {
                // Extract metadata information from matched PDF file
                let metadata: PDFMetadata = match run_analysis(&doc, &search_term) {
                    Ok(s) => s,
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(1);
                    }
                };
                println!("[{}] [INFO] File Name: {}", Utc::now(), &doc);
                metadata_vec.push(metadata);
            }
        }
    }
    else {
        let mut match_doc_flag: bool = false;

        // Traverse the matched PDF documents (containing the search term) to display the metadata information
        for doc in matched_docs {
            if doc == file_or_directory {
                let metadata: PDFMetadata = match run_analysis(&doc, &search_term) {
                    Ok(s) => s,
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(1);
                    }
                };

                match_doc_flag = true;
                println!("File Name: {}", &doc);
                metadata_vec.push(metadata);
                
                break;
            }
        }

        if match_doc_flag == false {
            println!("No matching documents founds.")
        }
    }

    Ok(metadata_vec)
}

#[pymodule]
/// A Python module implemented in Rust.
fn pypdf_seekers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(indexing_contents, m)?)?;
    m.add_function(wrap_pyfunction!(search_term_in_file, m)?)?;
    Ok(())
}