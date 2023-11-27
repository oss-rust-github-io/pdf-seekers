use clap::Parser;
use pdf_seekers::*;

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
    /// Keyword to be searched in PDF files (only required when action=Searching)
    search_term: Option<String>,
    
    #[clap(long)]
    /// Directory path where all indexed files, log files, and tracker files will be stored
    /// If no value is provided, then this will be created in current working directory
    cache_path: Option<String>,

    #[clap(long)]
    /// Flag to indicate whether to display processing logs on screen or not
    /// Default value is set to False
    display_logs: Option<bool>,
}

fn validate_arguments(args: &Arguments) -> String {
    if args.action.trim().len() == 0 {
        panic!("action cannot contain empty values");
    }

    if args.file_or_directory.trim().len() == 0 {
        panic!("file_or_directory cannot contain empty values");
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

fn main() {
    // Read command line arguments
    let args: Arguments = Arguments::parse();

    // Validate command line arguments
    let search_term: String = validate_arguments(&args);

    // Indexing the PDF files
    if &args.action == "index" {
        indexing_contents(&args.file_or_directory, args.cache_path.clone(), args.display_logs).unwrap();
    }

    // Search for provided keyword
    if &args.action == "search" {
        let metadata_vec = search_term_in_file(&args.file_or_directory, search_term, args.cache_path.clone(), args.display_logs).unwrap();

        for element in metadata_vec {
            element.show();
        }
    }
}