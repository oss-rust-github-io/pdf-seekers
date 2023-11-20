//! # Search Operations module
//! 
//! - Defines the metadata structure to be captured
//! - Defines the supporting functions for performing keyword search on PDF files
//! - Defines the supporting functions for capturing metadata information from matched PDF files

use crate::error::{FileOperationsError, SearchingError};
use lopdf::Document as lopdoc;
use itertools::Itertools;
use chrono::prelude::Utc;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;

#[derive(Debug)]
/// Defines the metadata for extracted information from PDF files
pub struct PDFMetadata {
    /// Number of pages in the PDF file
    num_pages: usize,
    /// Matched page numbers containing the search term
    matched_page_nums: Vec<u32>,
    /// Surrounding texts around the search term
    cropped_texts: Vec<String>
}

impl PDFMetadata {
    /// Constructor for building the `PDFMetadata` struct
    pub fn new(num_pages: usize, matched_page_nums: Vec<u32>, cropped_texts: Vec<String>) -> PDFMetadata {
        PDFMetadata {
            num_pages,
            matched_page_nums,
            cropped_texts
        }
    }

    /// Displays the metadata information based on search performed on PDF files
    pub fn show(&self) {
        println!("[{}] [INFO] Number of pages: {}", Utc::now(), self.num_pages);
        println!("[{}] [INFO] Search Term:", Utc::now());

        for (idx, page) in self.matched_page_nums.iter().enumerate(){
            println!("[{}] [INFO] Page: {}", Utc::now(), page);
            println!("[{}] [INFO] Extracted Text: {}", Utc::now(), self.cropped_texts[idx]);
        }
    }
}

/// Searches the given keyword in indexed files
/// 
/// ## Input Parameters
/// - `index` contains the Tantivy index for performing the search
/// - `query_str` contains the keyword to be searched in PDF files
/// 
/// ## Returns
/// - Vector containing matched PDF documents (containing the search term)
pub fn search_keyword(index: &tantivy::Index, query_str: &str) -> Result<Vec<String>, SearchingError> {
    // Create the index reader object
    let indexer = match index.reader() {
        Ok(s) => s,
        Err(e) => return Err(SearchingError::IndexReaderCreateError(e))
    };

    // Create the index searcher object
    let searcher = indexer.searcher();

    // Define the index field for running the search on
    let content_field = match index.schema().get_field("content") {
        Ok(s) => s,
        Err(e) => return Err(SearchingError::IndexFieldNotFound(String::from("content"), e))
    };
    let query_parser = QueryParser::for_index(&index, vec![content_field]);

    // Parse the query string
    let query = match query_parser.parse_query(query_str) {
        Ok(s) => s,
        Err(e) => return Err(SearchingError::QueryParserError(e))
    };

    // Search the index
    let top_docs = match searcher.search(&query, &TopDocs::with_limit(10)) {
        Ok(s) => s,
        Err(e) => return Err(SearchingError::KeywordSearchError(e))
    };

    // Capture search results
    let mut matched_docs: Vec<String> = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = match searcher.doc(doc_address) {
            Ok(s) => s,
            Err(e) => return Err(SearchingError::SearcherDocumentFetchError(e))
        };

        let path_field = match index.schema().get_field("path") {
            Ok(s) => s,
            Err(e) => return Err(SearchingError::IndexFieldNotFound(String::from("path"), e))
        };

        let path_value = retrieved_doc
            .get_first(path_field)
            .and_then(|v| v.as_text())
            .unwrap_or_default()
            .to_string();

        matched_docs.push(path_value);
    }

    let matched_docs: Vec<String> = matched_docs.into_iter().unique().collect();
    Ok(matched_docs)
}

/// Captures metadata information from PDF files based on search term provided
/// 
/// ## Input Parameters
/// - `file` contains the PDF file for information extration
/// - `keyword` contains the search term for extracting metadata information
/// 
/// ## Returns
/// - `PDFMetadata` struct containing captured metadata information for matched PDF files (containing the search term)
pub fn run_analysis(file: &String, keyword: &str) -> Result<PDFMetadata, FileOperationsError> {
    // Read the PDF file
    let doc = match lopdoc::load(&file) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::PDFFileReadError(file.clone(), e))
    };

    // Get all pages in the PDF file
    let pages = doc.get_pages();
    let num_pages: usize = pages.len();
    let mut matched_page_nums: Vec<u32> = Vec::new();
    let mut cropped_texts: Vec<String> = Vec::new();

    // Traverse through the PDF pages to extract metadata
    for (i, _) in pages.iter().enumerate() {
        let page_number: u32 = (i + 1) as u32;

        // Extract text from a single PDF page
        let text: String = match doc.extract_text(&[page_number]) {
            Ok(s) => s,
            Err(e) => return Err(FileOperationsError::PDFFileTextExtractionError(file.clone(), page_number, e))
        };
        
        // Extract surrounding text around the search term
        if text.contains(keyword) {
            matched_page_nums.push(page_number);

            let str_vec: Vec<&str> = text.split(' ').collect::<Vec<&str>>();
            let index: i32 = str_vec.iter().position(|&r| r == keyword).unwrap() as i32;
            
            let lower_bound: i32 = std::cmp::max(0, index-20);
            let upper_bound: i32 = std::cmp::min(index+21, str_vec.len() as i32);

            let cropped_vec: &[&str] = &str_vec[lower_bound as usize..upper_bound as usize];
            let cropped_text: String = cropped_vec.join(" ");
            cropped_texts.push(cropped_text)
        }
    }

    Ok(PDFMetadata::new(num_pages, matched_page_nums, cropped_texts))
}