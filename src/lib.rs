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

use chrono::Datelike;
use chrono::prelude::Utc;
use itertools::Itertools;
use std::collections::HashMap;

/// Create indexes for either single PDF file or directory containing multiple PDF files
/// 
/// ## Input Parameters
/// - `file_or_directory` defines the input path for single PDF file or directory containing multiple PDF files
/// - `cache_path` defines the input path for storing the indexed files, log files, and tracker files
/// - `display_logs` defines the flag to indicate whether to display processing logs on screen or not
pub fn indexing_contents(file_or_directory: &str, cache_path: Option<String>, display_logs: Option<bool>) -> Result<(), std::io::Error> {
    // Get cache directory path
    let cache_dir: String = match get_cache_dir(&cache_path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Create the directory for storing cache files (if doesn't exist)
    match create_dir_if_not_exists(&cache_dir) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let index_path: String = format!("{}/index_dir", &cache_dir);
    let log_path: String = format!("{}/logs_dir", &cache_dir);

    // Create the directory for storing logs files (if doesn't exist)
    match create_dir_if_not_exists(&log_path) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Define the log file name
    let dt = Utc::now();
    let log_file: String = format!("{}/pdf_seekers_{}_{}_{}.log", &log_path, dt.year(), dt.month(), dt.date_naive());

    let log_message: String = format!("[{}] \t[INFO] \t==================================================", Utc::now());
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \tIndexing Operation Logs", Utc::now());
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \t==================================================", Utc::now());
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \tInput Parameters:", Utc::now());
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \tfile_or_directory: {}", Utc::now(), &file_or_directory);
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();
    
    let log_message: String = format!("[{}] \t[INFO] \tcache_path: {:?}", Utc::now(), &cache_path);
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \tdisplay_logs: {:?}", Utc::now(), &display_logs);
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    // Check if indexing to be performed on single file or directory of files
    let dir_flag: bool = check_if_directory(file_or_directory);
    let log_message: String = format!("[{}] \t[DEBUG] \t{} - Directory Flag is {}", Utc::now(), file_or_directory, dir_flag);
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    if dir_flag {
        // Get all file names in directory
        let files_list: Vec<String> = match get_files_in_directory(file_or_directory) {
            Ok(s) => {
                let log_message: String = format!("[{}] \t[INFO] \t{} - Read all file names successfully in directory {}", Utc::now(), file_or_directory, dir_flag);
                print_log_to_screen(&display_logs, &log_message);
                write_to_file(&log_file, log_message).unwrap();
                s
            },
            Err(err) => {
                eprintln!("{}", &err);
                write_to_file(&log_file, err.to_string()).unwrap();
                std::process::exit(1);
            }
        };
        
        // Run indexing on all files in directory
        for file in &files_list {
            let log_message: String = format!("[{}] \t[INFO] \t{} - Indexing started...", Utc::now(), file);
            print_log_to_screen(&display_logs, &log_message);
            write_to_file(&log_file, log_message).unwrap();

            file_indexing(&file, &index_path[..], &log_file, &display_logs);

            let log_message: String = format!("[{}] \t[INFO] \t{} - Indexing completed.", Utc::now(), file);
            print_log_to_screen(&display_logs, &log_message);
            write_to_file(&log_file, log_message).unwrap();
        }
    }
    else {
        let log_message: String = format!("[{}] \t[INFO] \t{} - Indexing started...", Utc::now(), file_or_directory);
        print_log_to_screen(&display_logs, &log_message);
        write_to_file(&log_file, log_message).unwrap();
        
        // Run indexing on single file
        file_indexing(file_or_directory, &index_path[..], &log_file, &display_logs);

        let log_message: String = format!("[{}] \t[INFO] \t{} - Indexing completed.", Utc::now(), file_or_directory);
        print_log_to_screen(&display_logs, &log_message);
        write_to_file(&log_file, log_message).unwrap();
    }

    Ok(())
}

/// Search for a keyword in either single PDF file or directory containing multiple PDF files
/// 
/// ## Input Parameters
/// - `file_or_directory` defines the input path for single PDF file or directory containing multiple PDF files
/// - `search_term` defines the keyword to be searched in PDF documents
/// - `cache_path` defines the input path for storing the indexed files, log files, and tracker files
/// - `display_logs` defines the flag to indicate whether to display processing logs on screen or not
pub fn search_term_in_file(file_or_directory: &str, search_term: String, cache_path: Option<String>, display_logs: Option<bool>) -> Result<Vec<PDFMetadata>, std::io::Error> {
    // Get cache directory path
    let cache_dir: String = match get_cache_dir(&cache_path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Create the directory for storing cache files (if doesn't exist)
    match create_dir_if_not_exists(&cache_dir) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let index_path: String = format!("{}/index_dir", &cache_dir);
    let log_path: String = format!("{}/logs_dir", &cache_dir);

    // Create the directory for storing logs files (if doesn't exist)
    match create_dir_if_not_exists(&log_path) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Define the log file name
    let dt = Utc::now();
    let log_file: String = format!("{}/pdf_seekers_{}_{}_{}.log", &log_path, dt.year(), dt.month(), dt.date_naive());

    let log_message: String = format!("[{}] \t[INFO] \t==================================================", Utc::now());
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \tKeyword Search Operation Logs", Utc::now());
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \t==================================================", Utc::now());
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \tInput Parameters:", Utc::now());
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \tfile_or_directory: {}", Utc::now(), &file_or_directory);
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \tsearch_term: {}", Utc::now(), &search_term);
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();
    
    let log_message: String = format!("[{}] \t[INFO] \tcache_path: {:?}", Utc::now(), &cache_path);
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let log_message: String = format!("[{}] \t[INFO] \tdisplay_logs: {:?}", Utc::now(), &display_logs);
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();
    
    // Check if search to be performed on single file or directory of files
    let dir_flag: bool = check_if_directory(file_or_directory);
    let log_message: String = format!("[{}] \t[DEBUG] \t{} - Directory Flag is {}", Utc::now(), file_or_directory, dir_flag);
    print_log_to_screen(&display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();
    
    // Create or open the Tantivy index
    let index: tantivy::Index = match create_or_open_index(&index_path, &log_file, &display_logs) {
        Ok(s) => {
            let log_message: String = format!("[{}] \t[INFO] \tIndex writer created successfully for {}", Utc::now(), index_path);
            print_log_to_screen(&display_logs, &log_message);
            write_to_file(&log_file, log_message).unwrap();
            s
        },
        Err(err) => {
            eprintln!("{}", err);
            write_to_file(&log_file, err.to_string()).unwrap();
            std::process::exit(1);
        }
    };

    // Search for a term in the indexed PDFs
    let matched_docs: HashMap<String, Vec<String>> = match search_keyword(&index, &search_term) {
        Ok(s) => {
            let log_message: String = format!("[{}] \t[INFO] \tRetrieved matched documents successfully for `{}` search term", Utc::now(), search_term);
            print_log_to_screen(&display_logs, &log_message);
            write_to_file(&log_file, log_message).unwrap();
            s
        },
        Err(err) => {
            eprintln!("{}", err);
            write_to_file(&log_file, err.to_string()).unwrap();
            std::process::exit(1);
        }
    };

    // Run analysis on PDF documents containing the search term
    let mut metadata_vec: Vec<PDFMetadata> = Vec::new();
    if dir_flag {
        // Get all file names in directory
        let files_list: Vec<String> = match get_files_in_directory(file_or_directory) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("{}", err);
                write_to_file(&log_file, err.to_string()).unwrap();
                std::process::exit(1);
            }
        };
        
        // Traverse the matched PDF documents (containing the search term) to display the metadata information
        for doc_name in matched_docs.keys().sorted() {
            if files_list.contains(doc_name) {
                let page_num: Vec<String> = matched_docs.get(doc_name).cloned().unwrap();

                // Extract metadata information from matched PDF file
                let metadata: PDFMetadata = match run_analysis(doc_name, &page_num, &search_term) {
                    Ok(s) => s,
                    Err(err) => {
                        eprintln!("{}", err);
                        write_to_file(&log_file, err.to_string()).unwrap();
                        std::process::exit(1);
                    }
                };

                let log_message: String = format!("[{}] \t[INFO] \t{}: Metadata extracted successfully", Utc::now(), &doc_name);
                print_log_to_screen(&display_logs, &log_message);
                write_to_file(&log_file, log_message).unwrap();
                
                metadata_vec.push(metadata);
            }
        }
    }
    else {
        let mut match_doc_flag: bool = false;

        // Traverse the matched PDF documents (containing the search term) to display the metadata information
        for doc_name in matched_docs.keys().sorted() {
            if doc_name == file_or_directory {
                let page_num: Vec<String> = matched_docs.get(doc_name).cloned().unwrap();

                // Extract metadata information from given PDF file
                let metadata: PDFMetadata = match run_analysis(doc_name, &page_num, &search_term) {
                    Ok(s) => s,
                    Err(err) => {
                        eprintln!("{}", err);
                        write_to_file(&log_file, err.to_string()).unwrap();
                        std::process::exit(1);
                    }
                };

                let log_message: String = format!("[{}] \t[INFO] \t{}: Metadata extracted successfully", Utc::now(), &doc_name);
                print_log_to_screen(&display_logs, &log_message);
                write_to_file(&log_file, log_message).unwrap();

                metadata_vec.push(metadata);
                match_doc_flag = true;
                break;
            }
        }

        if match_doc_flag == false {
            println!("[{}] \t[INFO] \tNo matching documents found", Utc::now());
        }
    }

    Ok(metadata_vec)
}