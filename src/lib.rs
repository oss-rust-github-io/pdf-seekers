pub mod error;
pub mod file_operations;
pub mod index_operations;
pub mod search_operations;

use file_operations::*;
use index_operations::*;
use search_operations::*;

pub fn indexing_contents(file_or_directory: &str, index_path: &str) {
    // Check if search or indexing to be performed on single file or directory of files
    let dir_flag: bool = check_if_directory(file_or_directory);

    // Get all file names in directory
    if dir_flag {
        let files_list = match get_files_in_directory(file_or_directory) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

        for file in &files_list {
            file_indexing(&file, index_path);
        }
    }
    else {
        file_indexing(file_or_directory, index_path);
    }
}

pub fn search_term_in_file(index_path: &str, search_term: &String) {
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
    for doc in matched_docs {
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