use std::collections::HashMap;
use clap::Parser;
use pdf_seekers::{
    PDFMetadata,
    read_pdf, 
    create_or_open_index, 
    parse_and_index_pdf, 
    search_keyword, 
    run_analysis
};

#[derive(Parser, Default, Debug)]
#[clap(author="Tapas Das <dlaststark@gmail.com>", version, about)]
/// Simple parser and information extractor from PDF documents based on keyword search functionality (powered by Rust)
struct Arguments {
    #[clap(short, long)]
    /// Action to be performed [index, search]
    action: String,

    #[clap(short, long)]
    /// Provide single PDF file to be searched, or directory path containing multiple PDF files
    file_or_directory: String,
    
    #[clap(short, long)]
    /// Directory path where all indexed files will be stored
    index_path: String,

    #[clap(short, long)]
    /// Keyword to be searched in PDF files (only required when action=Searching)
    search_term: Option<String>,
}

fn validate_arguments(args: &Arguments) -> String {
    if args.action.trim().len() == 0 {
        panic!("action cannot contain empty values");
    }

    if args.file_or_directory.trim().len() == 0 {
        panic!("file_or_directory cannot contain empty values");
    }

    if args.index_path.trim().len() == 0 {
        panic!("index_path cannot contain empty values");
    }

    let mut search_term: String = String::from("");
    if args.action.trim().to_lowercase() == "search" {
        search_term = match args.search_term.as_ref() {
            Some(s) => s.to_string(),
            None => {
                panic!("search_term is not provided for 'search' action");
            }
        };

        if search_term.trim().len() == 0 {
            panic!("search_term cannot contain empty values for 'search' action");
        }
    }

    search_term
}

fn check_if_directory(file_or_directory: &str) -> bool {
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

fn file_indexing(file_path: &str, index_path: &str) {
    // Read text in PDF file
    let pdf_text:String = read_pdf(file_path);

    // Create or open the Tantivy index
    let index: tantivy::Index = create_or_open_index(index_path);

    // Parse PDF and index content
    parse_and_index_pdf(file_path, pdf_text, &index);

    println!("{} - Indexing completed.", file_path);
}

fn search_term_in_file(index_path: &str, search_term: &String) {
    // Create or open the Tantivy index
    let index: tantivy::Index = create_or_open_index(index_path);

    // Search for a term in the indexed PDFs
    let matched_docs: Vec<String> = search_keyword(&index, search_term);

    // Run analysis on PDF documents containing the search term
    for doc in matched_docs {
        let metadata: HashMap<String, PDFMetadata> = run_analysis(&doc, search_term);
        println!("File Name: {}", &doc);
        metadata.get(&doc).unwrap().show();
    }
}

fn main() {
    // Read command line arguments
    let args: Arguments = Arguments::parse();

    // Validate command line arguments
    let search_term: String = validate_arguments(&args);

    // Check if search or indexing to be performed on single file or directory of files
    let dir_flag: bool = check_if_directory(&args.file_or_directory);

    // Get all file names in directory
    let mut files_list: Vec<String> = Vec::new();
    if dir_flag {
        let file_paths = std::fs::read_dir(&args.file_or_directory).unwrap_or_else(|err| {
            panic!("{} - Unable to open given directory", err);
        });

        for file in file_paths {
            let file_name = file.unwrap().file_name().into_string().unwrap();
            files_list.push(format!("{}/{}",&args.file_or_directory, file_name));
        }
    }

    if &args.action == "index" {
        if dir_flag {
            for file in &files_list {
                file_indexing(&file, &args.index_path);
            }
        }
        else {
            file_indexing(&args.file_or_directory, &args.index_path);
        }
    }

    if &args.action == "search" {
        search_term_in_file(&args.index_path, &search_term);
    }
}
