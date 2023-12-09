//! # Search Operations module
//! 
//! - Defines the metadata structure to be captured
//! - Defines the supporting functions for performing keyword search on PDF files
//! - Defines the supporting functions for capturing metadata information from matched PDF files

use crate::error::{FileOperationsError, SearchingError};
use log::{debug, trace};
use lopdf::Document as lopdoc;
use std::collections::HashMap;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;

#[derive(Debug)]
/// Defines the metadata for extracted information from PDF files
pub struct PDFMetadata {
    /// Name of the PDF file
    pub doc_name: String,
    /// Number of pages in the PDF file
    pub num_pages: usize,
    /// Matched page numbers containing the search term
    pub matched_page_nums: Vec<u32>,
    /// Surrounding texts around the search term
    pub cropped_texts: Vec<String>
}

impl PDFMetadata {
    /// Displays the metadata information based on search performed on PDF files
    pub fn show(&self) {
        println!("==================================================");
        println!("Document Name: {}", self.doc_name);
        println!("Number of pages: {}", self.num_pages);
        println!("Search Results:");

        for (idx, page) in self.matched_page_nums.iter().enumerate(){
            println!("[Page: {}] {}", page, self.cropped_texts[idx]);
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
/// - Hashmap containing matched PDF documents and corresponding page numbers (containing the search term)
pub fn search_keyword(index: &tantivy::Index, query_str: &str) -> Result<HashMap<String, Vec<String>>, SearchingError> {
    // Create the index reader object
    let indexer = match index.reader() {
        Ok(s) => s,
        Err(e) => return Err(SearchingError::IndexReaderCreateError(e))
    };

    debug!(target:"other_logging", "Index reader object created successfully.");

    // Create the index searcher object
    let searcher = indexer.searcher();
    debug!(target:"other_logging", "Index searcher object created successfully.");

    // Define the index field for running the search on
    let content_field = match index.schema().get_field("content") {
        Ok(s) => s,
        Err(e) => return Err(SearchingError::IndexFieldNotFound(String::from("content"), e))
    };
    let query_parser = QueryParser::for_index(&index, vec![content_field]);
    debug!(target:"other_logging", "Query parser created successfully for `content` field.");

    // Parse the query string
    let query = match query_parser.parse_query(query_str) {
        Ok(s) => s,
        Err(e) => return Err(SearchingError::QueryParserError(e))
    };
    debug!(target:"other_logging", "Query parsing completed successfully for query string -> {}", &query_str);

    // Search the index
    let top_docs = match searcher.search(&query, &TopDocs::with_limit(10)) {
        Ok(s) => s,
        Err(e) => return Err(SearchingError::KeywordSearchError(e))
    };
    debug!(target:"other_logging", "Top 10 matched documents retrived from search.");

    // Capture search results
    let mut doc_page_map: HashMap<String, Vec<String>> = HashMap::new();

    for (_score, doc_address) in top_docs {
        let retrieved_doc = match searcher.doc(doc_address) {
            Ok(s) => s,
            Err(e) => return Err(SearchingError::SearcherDocumentFetchError(e))
        };

        let path_field = match index.schema().get_field("path") {
            Ok(s) => s,
            Err(e) => return Err(SearchingError::IndexFieldNotFound(String::from("path"), e))
        };

        let page_num_field = match index.schema().get_field("page_num") {
            Ok(s) => s,
            Err(e) => return Err(SearchingError::IndexFieldNotFound(String::from("page_num"), e))
        };

        let doc_name = retrieved_doc
            .get_first(path_field)
            .and_then(|v| v.as_text())
            .unwrap_or_default()
            .to_string();

        let mut page_num: Vec<String> = Vec::new();
        for item in retrieved_doc.get_all(page_num_field) {
            page_num.push(item.as_text().unwrap().to_string());
        }
        
        doc_page_map.insert(doc_name, page_num);
    }
    
    Ok(doc_page_map)
}

/// Captures metadata information from PDF files based on search term provided
/// 
/// ## Input Parameters
/// - `file` contains the PDF file for information extration
/// - 
/// - `keyword` contains the search term for extracting metadata information
/// - `page_num` contains the matched page numbers in PDF document containing the search term
/// 
/// ## Returns
/// - `PDFMetadata` struct containing captured metadata information for matched PDF files (containing the search term)
pub fn run_analysis(file: &String, page_num: &Vec<String>, keyword: &str) -> Result<PDFMetadata, FileOperationsError> {
    // Read the PDF file
    let doc = match lopdoc::load(file) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::PDFFileReadError(file.clone(), e))
    };

    trace!(target:"other_logging", "PDF file `{}` with {} pages read successfully.", &file, &doc.get_pages().len());

    // Get all pages in the PDF file
    let pages = doc.get_pages();
    let num_pages: usize = pages.len();
    let mut matched_page_nums: Vec<u32> = Vec::new();
    let mut cropped_texts: Vec<String> = Vec::new();

    for item in page_num {
        let p_num: u32 = item.trim().parse::<u32>().unwrap();

        // Extract text from matched PDF pages containing the search term
        let text: String = match doc.extract_text(&[p_num]) {
            Ok(s) => s,
            Err(e) => return Err(FileOperationsError::PDFFileTextExtractionError(file.clone(), p_num, e))
        };

        // Extract surrounding text around the search term
        if text.contains(keyword) {
            matched_page_nums.push(p_num);

            let str_vec: Vec<&str> = text.split(' ').collect::<Vec<&str>>();
            let index: i32 = str_vec.iter().position(|&r| r == keyword).unwrap() as i32;
            
            let lower_bound: i32 = std::cmp::max(0, index-20);
            let upper_bound: i32 = std::cmp::min(index+21, str_vec.len() as i32);

            let cropped_vec: &[&str] = &str_vec[lower_bound as usize..upper_bound as usize];
            let cropped_text: String = cropped_vec.join(" ");

            cropped_texts.push(cropped_text)
        }
    }

    Ok(PDFMetadata{
        doc_name: file.clone(),
        num_pages, 
        matched_page_nums, 
        cropped_texts
    })
}