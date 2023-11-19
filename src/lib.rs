pub mod error;
use error::{FileOperationsError, IndexingError, SearchingError};
use lopdf::Document as lopdoc;
use itertools::Itertools;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{Index, IndexWriter, Document};
use tantivy::schema::{SchemaBuilder, TEXT, STORED, STRING};


#[derive(Debug)]
pub struct PDFMetadata {
    num_pages: usize,
    matched_page_nums: Vec<u32>,
    cropped_texts: Vec<String>
}

impl PDFMetadata {
    pub fn new(num_pages: usize, matched_page_nums: Vec<u32>, cropped_texts: Vec<String>) -> PDFMetadata {
        PDFMetadata {
            num_pages,
            matched_page_nums,
            cropped_texts
        }
    }

    pub fn show(&self) {
        println!("Number of pages: {}", self.num_pages);
        println!("Search Term: \n");

        for (idx, page) in self.matched_page_nums.iter().enumerate(){
            println!("   Page: {} \nExtracted Text: {}\n", page, self.cropped_texts[idx]);
        }
    }
}

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

pub fn create_or_open_index(index_path: &str) -> Result<Index, IndexingError> {
    match std::fs::create_dir_all(index_path) {
        Ok(_) => {},
        Err(e) => return Err(IndexingError::IndexDirectoryCreateError(index_path.to_string(), e))
    };

    let dir_content = match std::fs::read_dir(index_path) {
        Ok(s) => s,
        Err(e) => return Err(IndexingError::IndexDirectoryReadError(index_path.to_string(), e))
    };

    let dir_check: bool = dir_content.count() == 0;

    let index: Index = if !dir_check {
        match Index::open_in_dir(index_path) {
            Ok(s) => s,
            Err(e) => return Err(IndexingError::IndexDirectoryOpenError(index_path.to_string(), e))
        }
    } else {
        let mut schema_builder: SchemaBuilder = SchemaBuilder::new();

        // Add fields to the schema
        schema_builder.add_text_field("content", TEXT | STORED);
        schema_builder.add_text_field("path", STRING | STORED);

        let index: Index = match Index::builder()
            .schema(schema_builder.build())
            .create_in_dir(index_path) {
                Ok(s) => s,
                Err(e) => return Err(IndexingError::IndexCreateError(index_path.to_string(), e))
            };

        // Set up the index writer
        let index_writer: IndexWriter = match index
            .writer(50_000_000) { // 50MB heap size for indexing
                Ok(s) => s,
                Err(e) => return Err(IndexingError::IndexWriterCreateError(e))
            };

        // Close the index writer
        drop(index_writer);

        index
    };

    Ok(index)
}

pub fn parse_and_index_pdf(pdf_file: &str, pdf_text: String, index: &Index) -> Result<(), IndexingError> {
    // Create a Tantivy index writer
    let mut index_writer = match index
        .writer_with_num_threads(1, 40_000_000) {
            Ok(s) => s,
            Err(e) => return Err(IndexingError::IndexWriterCreateError(e))
        };

    // Create a Tantivy document
    let mut doc = Document::default();

    // Add PDF content and path to the index document
    doc.add_text(match index.schema().get_field("content") {
        Ok(s) => s,
        Err(e) => return Err(IndexingError::IndexFieldNotFound(String::from("content"), e))
    }, &pdf_text);

    doc.add_text(
        match index.schema().get_field("path") {
            Ok(s) => s,
            Err(e) => return Err(IndexingError::IndexFieldNotFound(String::from("path"), e))
        },
        &pdf_file.to_string(),
    );

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

pub fn search_keyword(index: &Index, query_str: &str) -> Result<Vec<String>, SearchingError> {
    let indexer = match index.reader() {
        Ok(s) => s,
        Err(e) => return Err(SearchingError::IndexReaderCreateError(e))
    };

    let searcher = indexer.searcher();
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

pub fn run_analysis(file: &String, keyword: &str) -> Result<PDFMetadata, FileOperationsError> {
    let doc = match lopdoc::load(&file) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::PDFFileReadError(file.clone(), e))
    };

    let pages = doc.get_pages();
    let num_pages: usize = pages.len();
    let mut matched_page_nums: Vec<u32> = Vec::new();
    let mut cropped_texts: Vec<String> = Vec::new();

    for (i, _) in pages.iter().enumerate() {
        let page_number: u32 = (i + 1) as u32;
        let text: String = match doc.extract_text(&[page_number]) {
            Ok(s) => s,
            Err(e) => return Err(FileOperationsError::PDFFileTextExtractionError(file.clone(), page_number, e))
        };
        
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

pub fn check_if_directory(file_or_directory: &str) -> bool {
    let split_dir: Vec<&str> = file_or_directory.split('/').collect::<Vec<&str>>();

    let last_item: &str = split_dir[split_dir.len()-1];
    let split_item: Vec<&str> = last_item.split('.').collect::<Vec<&str>>();

    if split_item.len() == 1 {
        return true
    }
    else {
        return false
    }
}

fn get_files_in_directory(directory: &str) -> Result<Vec<String>, FileOperationsError> {
    let mut files_list: Vec<String> = Vec::new();

    let file_paths = match std::fs::read_dir(directory) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::DirectoryReadError(directory.to_string(), e))
    };

    for file in file_paths {
        let file_name = file.unwrap().file_name().into_string().unwrap();
        files_list.push(format!("{}/{}",directory, file_name));
    }

    Ok(files_list)
}

fn file_indexing(file_path: &str, index_path: &str) {
    // Read text in PDF file
    let pdf_text:String = match read_pdf(file_path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Create or open the Tantivy index
    let index: tantivy::Index = match create_or_open_index(index_path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Parse PDF and index content
    match parse_and_index_pdf(file_path, pdf_text, &index) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    println!("{} - Indexing completed.", file_path);
}

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