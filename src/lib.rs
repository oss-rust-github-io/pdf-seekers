pub mod error;
pub mod file_operations;
pub mod index_operations;
pub mod search_operations;

use file_operations::*;
use index_operations::*;
use search_operations::*;

pub fn indexing_contents(file_or_directory: &str, index_path: &str) {
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
        }
    }
    else {
        // Run indexing on single file
        file_indexing(file_or_directory, index_path);
    }
}

pub fn search_term_in_file(file_or_directory: &str, index_path: &str, search_term: &String) {
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
    let matched_docs: Vec<String> = match search_keyword(&index, search_term) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Run analysis on PDF documents containing the search term
    if dir_flag {
        // Get all file names in directory
        let files_list = match get_files_in_directory(file_or_directory) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

        for doc in matched_docs {
            if files_list.contains(&doc) {
                let metadata: PDFMetadata = match run_analysis(&doc, search_term) {
                    Ok(s) => s,
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(1);
                    }
                };
                println!("File Name: {}", &doc);
                metadata.show();
            }
        }
    }
    else {
        let mut match_doc_flag: bool = false;

        for doc in matched_docs {
            if doc == file_or_directory {
                let metadata: PDFMetadata = match run_analysis(&doc, search_term) {
                    Ok(s) => s,
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(1);
                    }
                };

                match_doc_flag = true;
                println!("File Name: {}", &doc);
                metadata.show();
                
                break;
            }
        }

        if match_doc_flag == false {
            println!("No matching documents founds.")
        }
    }
}