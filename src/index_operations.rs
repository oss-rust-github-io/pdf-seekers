//! # Index Operations module
//! 
//! - Defines the supporting functions for parsing and indexing PDF files
//! - Creates the directory for storing indexed files, if it doesn't exist

use chrono::prelude::Utc;
use crate::file_operations::*;
use crate::error::IndexingError;
use tantivy::{Index, IndexWriter, Document};
use tantivy::schema::{SchemaBuilder, TEXT, STORED, STRING};

const NUM_THREADS: usize = 1;
const OVERALL_MEMORY_ARENA_IN_BYTES: usize = 1 << 30; // 1GiB

/// Creates or opens the directory to be used for storing indexed files
/// 
/// ## Input Parameters
/// - `index_path` defines the input path for storing the indexed files
/// - `log_file` defines the input path for storing the log files
/// - `display_logs` defines the flag to indicate whether to display processing logs on screen or not
/// 
/// ## Returns
/// - Tantivy index for performing keyword search on PDF files
pub fn create_or_open_index(index_path: &str, log_file: &String, display_logs: &Option<bool>) -> Result<Index, IndexingError> {
    // Create the directory for storing indexed files (if doesn't exist)
    match std::fs::create_dir_all(index_path) {
        Ok(_) => {
            let log_message: String = format!("[{}] \t[DEBUG] \tIndex directory created successfully at {}", Utc::now(), index_path);
            print_log_to_screen(display_logs, &log_message);
            write_to_file(&log_file, log_message).unwrap();
        },
        Err(e) => return Err(IndexingError::IndexDirectoryCreateError(index_path.to_string(), e))
    };

    // Read contents of the index directory
    let dir_content = match std::fs::read_dir(index_path) {
        Ok(s) => s,
        Err(e) => return Err(IndexingError::IndexDirectoryReadError(index_path.to_string(), e))
    };

    let dir_check: bool = dir_content.count() == 0;

    let log_message: String = format!("[{}] \t[DEBUG] \t{} - Is Index directory empty? {}", Utc::now(), index_path, dir_check);
    print_log_to_screen(display_logs, &log_message);
    write_to_file(&log_file, log_message).unwrap();

    let index: Index = if !dir_check {
        // Open the index directory to build the Tantivy index (if directory is not empty)
        match Index::open_in_dir(index_path) {
            Ok(s) => {
                let log_message: String = format!("[{}] \t[DEBUG] \tRead contents successfully of Index directory {}", Utc::now(), index_path);
                print_log_to_screen(display_logs, &log_message);
                write_to_file(&log_file, log_message).unwrap();
                s
            },
            Err(e) => return Err(IndexingError::IndexDirectoryOpenError(index_path.to_string(), e))
        }
    } else {
        let mut schema_builder: SchemaBuilder = SchemaBuilder::new();

        // Add fields to the schema
        schema_builder.add_text_field("content", TEXT | STORED);
        schema_builder.add_text_field("path", STRING | STORED);
        schema_builder.add_text_field("page_num", STRING | STORED);

        // Build the Tantivy index (if index directory is empty)
        let index: Index = match Index::builder()
            .schema(schema_builder.build())
            .create_in_dir(index_path) {
                Ok(s) => s,
                Err(e) => return Err(IndexingError::IndexCreateError(index_path.to_string(), e))
            };

        // Set up the index writer
        let index_writer: IndexWriter = match index
            .writer_with_num_threads(NUM_THREADS, OVERALL_MEMORY_ARENA_IN_BYTES) { // 50MB heap size for indexing
                Ok(s) => s,
                Err(e) => return Err(IndexingError::IndexWriterCreateError(e))
            };

        // Close the index writer
        drop(index_writer);

        index
    };

    Ok(index)
}

/// Parse a PDF file and create indexes for same in index directory
/// 
/// ## Input Parameters
/// - `pdf_file` contains the PDF file to be parsed and indexed
/// - `pdf_text` contains the extracted text from PDF file for indexing
/// - `index` contains the Tantivy index for parsing and indexing
/// 
/// ## Returns
/// - None
pub fn parse_and_index_pdf(pdf_file: &str, pdf_page_num: Vec<u32>, pdf_text: Vec<String>, index: &Index) -> Result<(), IndexingError> {
    // Create a Tantivy index writer
    let mut index_writer = match index
        .writer_with_num_threads(NUM_THREADS, OVERALL_MEMORY_ARENA_IN_BYTES) {
            Ok(s) => s,
            Err(e) => return Err(IndexingError::IndexWriterCreateError(e))
        };

    // Prevent any segment merge, again to control the number of segments.
    index_writer.set_merge_policy(Box::new(tantivy::merge_policy::NoMergePolicy));

    // Create a Tantivy document
    let mut doc = Document::default();

    // Add PDF content, path and page number to the index document
    for (idx, page_num) in pdf_page_num.iter().enumerate() {
        doc.add_text(match index.schema().get_field("content") {
            Ok(s) => s,
            Err(e) => return Err(IndexingError::IndexFieldNotFound(String::from("content"), e))
        }, &pdf_text[idx]);

        doc.add_text(match index.schema().get_field("page_num") {
            Ok(s) => s,
            Err(e) => return Err(IndexingError::IndexFieldNotFound(String::from("page_num"), e))
        }, page_num);

        doc.add_text(
            match index.schema().get_field("path") {
                Ok(s) => s,
                Err(e) => return Err(IndexingError::IndexFieldNotFound(String::from("path"), e))
            },
            &pdf_file.to_string(),
        );
    }

    // Add the document to the index
    match index_writer.add_document(doc) {
        Ok(_) => {},
        Err(e) => return Err(IndexingError::IndexDocumentAddError(e))
    };

    // Commit changes to the index
    match index_writer.commit() {
        Ok(_) => {},
        Err(e) => return Err(IndexingError::IndexDocumentCommitError(e))
    };

    Ok(())
}

/// Parse and index either single PDF file or directory containing multiple PDF files
/// 
/// ## Input Parameters
/// - `file_path` defines the input path for single PDF file or directory containing multiple PDF files
/// - `index_path` defines the input path for storing the indexed files
/// - `log_file` defines the input path for storing the log files
/// - `display_logs` defines the flag to indicate whether to display processing logs on screen or not
/// 
/// ## Returns
/// - None
pub fn file_indexing(file_path: &str, index_path: &str, log_file: &String, display_logs: &Option<bool>) {
    // Read text in PDF file
    let (pdf_page_nums, pdf_texts) = match read_pdf(file_path) {
        Ok(s) => {
            let log_message: String = format!("[{}] \t[INFO] \t{} - File read successfully", Utc::now(), file_path);
            print_log_to_screen(display_logs, &log_message);
            write_to_file(&log_file, log_message).unwrap();
            s
        },
        Err(err) => {
            eprintln!("{}", err);
            write_to_file(&log_file, err.to_string()).unwrap();
            std::process::exit(1);
        }
    };

    // Create or open the Tantivy index
    let index: tantivy::Index = match create_or_open_index(index_path, log_file, display_logs) {
        Ok(s) => {
            let log_message: String = format!("[{}] \t[INFO] \tIndex writer created successfully for {}", Utc::now(), index_path);
            print_log_to_screen(display_logs, &log_message);
            write_to_file(&log_file, log_message).unwrap();
            s
        },
        Err(err) => {
            eprintln!("{}", err);
            write_to_file(&log_file, err.to_string()).unwrap();
            std::process::exit(1);
        }
    };

    // Parse PDF and index content
    match parse_and_index_pdf(file_path, pdf_page_nums, pdf_texts, &index) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}", err);
            write_to_file(&log_file, err.to_string()).unwrap();
            std::process::exit(1);
        }
    };
}