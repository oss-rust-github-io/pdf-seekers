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
pub mod logging;
pub mod file_operations;
pub mod index_operations;
pub mod search_operations;

use file_operations::*;
use index_operations::*;
use search_operations::*;

use chrono::prelude::Utc;
use itertools::Itertools;
use std::collections::HashMap;
use log::{info, debug, error, trace};


/// Create indexes for either single PDF file or directory containing multiple PDF files
/// 
/// ## Input Parameters
/// - `file_or_directory` defines the input path for single PDF file or directory containing multiple PDF files
/// - `cache_path` defines the input path for storing the indexed files, log files, and tracker files
/// - `log_level` defines the verbosity level for logs
pub fn indexing_contents(file_or_directory: String, cache_path: Option<String>, log_level: Option<String>) -> Result<(), std::io::Error> {
    // Create the directory for storing cache files (if doesn't exist)
    let cache_dir: String = match create_cache_dir_if_not_exists(&cache_path) {
        Ok(s) => s,
        Err(err) => {
            panic!("{}", err);
        }
    };

    // Define file paths for indexing, tracking and logging
    let index_path: String = format!("{}/index_dir", &cache_dir);
    let track_path: String = format!("{}/track_dir", &cache_dir);
    let track_success_file: String = format!("{}/_SUCCESS.txt", &track_path);
    let track_fail_file: String = format!("{}/_FAIL.txt", &track_path);
    let log_file: String = format!("{}/logs_dir/pdf_seekers_index-{}.log", &cache_dir, Utc::now().date_naive());

    // Create the directory for storing tracker files (if doesn't exist)
    match create_track_dir_if_not_exists(track_path) {
        Ok(_) => {},
        Err(err) => {
            error!(target:"other_logging", "{}", err);
            std::process::exit(1);
        }
    };

    // Setup log4rs handle
    let _log_handle = match logging::set_logging(&log_file, &log_level) {
        Ok(s) => s,
        Err(err) => {
            panic!("{}", err);
        }
    };

    info!(target:"info_logging", "Starting indexing operation...");
    debug!(target:"other_logging", "Input parameters:");
    debug!(target:"other_logging", "file_or_directory: {}", &file_or_directory);
    debug!(target:"other_logging", "cache_path: {:?}", &cache_path);
    debug!(target:"other_logging", "log_level: {:?}", &log_level);

    // Check if indexing to be performed on single file or directory of files
    let dir_flag: bool = check_if_directory(&file_or_directory);
    trace!(target:"other_logging", "dir_flag: {}", &dir_flag);

    if dir_flag {
        info!(target:"info_logging", "Received `{}` which is directory.", &file_or_directory);
    }
    else {
        info!(target:"info_logging", "Received `{}` which is file.", &file_or_directory);
    }

    match &cache_path {
        Some(s) => trace!(target:"other_logging", "Setting folder path `{}` as cache directory.", s),
        None => trace!(target:"other_logging", "No cache directory provided in input. Defaulting to `{}` directory.", &cache_dir)
    };

    match &log_level {
        Some(s) => trace!(target:"other_logging", "Setting log verbosity level to `{}`.", s.to_uppercase()),
        None => trace!(target:"other_logging", "No log verbosity level provided in input. Defaulting to `INFO` log verbosity.")
    }

    if dir_flag {
        // Create tracking files (if does not exists)
        match std::path::Path::new(&track_success_file).exists() {
            true => {},
            false => {
                write_to_file(&track_success_file, &String::from("")).unwrap();
            }
        }

        write_to_file(&track_fail_file, &String::from("")).unwrap();

        // Get all file names in directory
        let files_list: Vec<String> = match get_files_in_directory(&file_or_directory, Some(&track_fail_file)) {
            Ok(s) => s,
            Err(err) => {
                error!(target:"other_logging", "{}", err);
                std::process::exit(1);
            }
        };

        info!(target:"info_logging", "Read all file names successfully in directory `{}`", &file_or_directory);
        trace!(target:"other_logging", "File names read from `{}` directory -> {:?}", &file_or_directory, &files_list);

        // Get all processed file names (from prior indexing processes)
        let processed_file: Vec<String> = match read_from_file(&track_success_file) {
            Ok(s) => s,
            Err(err) => {
                error!(target:"other_logging", "{}", err);
                std::process::exit(1);
            }
        };

        info!(target:"info_logging", "Read all processed file names successfully in `{}`", &track_success_file);
        trace!(target:"other_logging", "Processed file names read from `{}` file -> {:?}", &track_success_file, &processed_file);

        // Run indexing on new files in directory
        for file in &files_list {
            if !processed_file.contains(&file){
                info!(target:"info_logging", "{} - Indexing started...", &file);
                file_indexing(&file, &index_path, Some(&track_fail_file));
                info!(target:"info_logging", "{} - Indexing completed successfully.", &file);
                write_to_file(&track_success_file, &file).unwrap();
            }
            else {
                info!(target:"info_logging", "{} - Index information already captured.", &file);
            }
        }

        // Get all errored out file names
        let error_files: Vec<String> = match read_from_file(&track_fail_file) {
            Ok(s) => s,
            Err(err) => {
                error!(target:"other_logging", "{}", err);
                std::process::exit(1);
            }
        };
        debug!("Errored out files during indexing process: {:?}", error_files);
    }
    else {
        // Run indexing on single file
        info!(target:"info_logging", "{} - Indexing started...", file_or_directory);
        file_indexing(&file_or_directory, &index_path, Some(&track_fail_file));
        info!(target:"info_logging", "{} - Indexing completed successfully.", &file_or_directory);
    }

    Ok(())
}

/// Search for a keyword in either single PDF file or directory containing multiple PDF files
/// 
/// ## Input Parameters
/// - `file_or_directory` defines the input path for single PDF file or directory containing multiple PDF files
/// - `search_term` defines the keyword to be searched in PDF documents
/// - `cache_path` defines the input path for storing the indexed files, log files, and tracker files
/// - `log_level` defines the verbosity level for logs
pub fn search_term_in_file(file_or_directory: String, search_term: String, cache_path: Option<String>, log_level: Option<String>) -> Result<Vec<PDFMetadata>, std::io::Error> {
    // Create the directory for storing cache files (if doesn't exist)
    let cache_dir: String = match create_cache_dir_if_not_exists(&cache_path) {
        Ok(s) => s,
        Err(err) => {
            panic!("{}", err);
        }
    };

    // Define file paths for indexing and logging
    let index_path: String = format!("{}/index_dir", &cache_dir);
    let log_file: String = format!("{}/logs_dir/pdf_seekers_index-{}.log", &cache_dir, Utc::now().date_naive());

    // Setup log4rs handle
    let _log_handle = match logging::set_logging(&log_file, &log_level) {
        Ok(s) => s,
        Err(err) => {
            panic!("{}", err);
        }
    };
    
    info!(target:"info_logging", "Starting searching operation...");
    debug!(target:"other_logging", "Input parameters:");
    debug!(target:"other_logging", "file_or_directory: {}", &file_or_directory);
    debug!(target:"other_logging", "search_term: {}", &search_term);
    debug!(target:"other_logging", "cache_path: {:?}", &cache_path);
    debug!(target:"other_logging", "log_level: {:?}", &log_level);

    // Check if search to be performed on single file or directory of files
    let dir_flag: bool = check_if_directory(&file_or_directory);
    trace!(target:"other_logging", "dir_flag: {}", &dir_flag);

    if dir_flag {
        info!(target:"info_logging", "Received `{}` which is directory.", &file_or_directory);
    }
    else {
        info!(target:"info_logging", "Received `{}` which is file.", &file_or_directory);
    }

    match &cache_path {
        Some(s) => trace!(target:"other_logging", "Setting folder path `{}` as cache directory.", s),
        None => trace!(target:"other_logging", "No cache directory provided in input. Defaulting to `{}` directory.", &cache_dir)
    };

    match &log_level {
        Some(s) => trace!(target:"other_logging", "Setting log verbosity level to `{}`.", s.to_uppercase()),
        None => trace!(target:"other_logging", "No log verbosity level provided in input. Defaulting to `INFO` log verbosity.")
    }
    
    // Create or open the Tantivy index
    let index: tantivy::Index = match create_or_open_index(&index_path) {
        Ok(s) => s,
        Err(err) => {
            error!(target:"other_logging", "{}", err);
            std::process::exit(1);
        }
    };

    info!(target:"info_logging", "Index writer created successfully for `{}`.", &index_path);

    // Search for a term in the indexed PDFs
    let matched_docs: HashMap<String, Vec<String>> = match search_keyword(&index, &search_term) {
        Ok(s) => {
            info!(target:"info_logging", "Retrieved matched documents successfully for `{}` search term.", &search_term);
            s
        },
        Err(err) => {
            error!(target:"other_logging", "{}", err);
            std::process::exit(1);
        }
    };

    // Run analysis on PDF documents containing the search term
    let mut metadata_vec: Vec<PDFMetadata> = Vec::new();
    if dir_flag {
        // Get all file names in directory
        let files_list: Vec<String> = match get_files_in_directory(&file_or_directory, None) {
            Ok(s) => s,
            Err(err) => {
                error!(target:"other_logging", "{}", err);
                std::process::exit(1);
            }
        };

        info!(target:"info_logging", "Read all file names successfully in directory `{}`", &file_or_directory);
        trace!(target:"other_logging", "File names read from `{}` directory -> {:?}", &file_or_directory, &files_list);
        
        // Traverse the matched PDF documents (containing the search term) to display the metadata information
        for doc_name in matched_docs.keys().sorted() {
            if files_list.contains(doc_name) {
                let page_num: Vec<String> = matched_docs.get(doc_name).cloned().unwrap();

                // Extract metadata information from matched PDF file
                let metadata: PDFMetadata = match run_analysis(doc_name, &page_num, &search_term) {
                    Ok(s) => s,
                    Err(err) => {
                        error!("{}", err);
                        std::process::exit(1);
                    }
                };

                info!(target:"info_logging", "{}: Metadata extracted successfully.", &doc_name);                
                trace!(target:"other_logging", "{}: {:?}", &doc_name, &metadata);
                metadata_vec.push(metadata);
            }
        }
    }
    else {
        let mut match_doc_flag: bool = false;

        // Traverse the matched PDF documents (containing the search term) to display the metadata information
        for doc_name in matched_docs.keys().sorted() {
            if doc_name == &file_or_directory {
                let page_num: Vec<String> = matched_docs.get(doc_name).cloned().unwrap();

                // Extract metadata information from given PDF file
                let metadata: PDFMetadata = match run_analysis(doc_name, &page_num, &search_term) {
                    Ok(s) => s,
                    Err(err) => {
                        error!("{}", err);
                        std::process::exit(1);
                    }
                };

                info!(target:"info_logging", "{}: Metadata extracted successfully.", &doc_name);
                trace!(target:"other_logging", "{}: {:?}", &doc_name, &metadata);

                metadata_vec.push(metadata);
                match_doc_flag = true;
                break;
            }
        }

        if match_doc_flag == false {
            info!(target:"info_logging", "No matching documents found");
        }
    }

    Ok(metadata_vec)
}