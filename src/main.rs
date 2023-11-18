use clap::Parser;
use pdf_seekers::{
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
    /// Keyword to be searched in PDF files (only required when action=Searching)
    search_term: Option<String>,
}

fn validate_arguments(args: &Arguments) {
    if args.action.trim().len() == 0 {
        panic!("action cannot contain empty values");
    }

    if args.file_or_directory.trim().len() == 0 {
        panic!("file_or_directory cannot contain empty values");
    }

    if args.action.trim().to_lowercase() == "search" {
        let search_term = match args.search_term.as_ref() {
            Some(s) => s,
            None => {
                panic!("search_term is not provided for 'search' action");
            }
        };

        if search_term.trim().len() == 0 {
            panic!("search_term cannot contain empty values for 'search' action");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read command line arguments
    let args: Arguments = Arguments::parse();
    println!("action: {} \nfile_or_directory: {} \nsearch_term: {:?}", args.action, args.file_or_directory, args.search_term);

    // Validate command line arguments
    validate_arguments(&args);

    // Specify the PDF file to be read
    let pdf_file: &str = "data/fast_rcnn.pdf";

    // Specify the keyword to be search
    let keyword: &str = "convolutional";

    // Read text in PDF file
    let pdf_text:String = read_pdf(pdf_file);

    // Create or open the Tantivy index
    let index: tantivy::Index = create_or_open_index("pdf_index");

    // Parse PDF and index content
    parse_and_index_pdf(pdf_file, pdf_text, &index);

    // Search for a term in the indexed PDFs
    let matched_docs: Vec<String> = search_keyword(&index, keyword);

    // Run analysis on PDF documents containing the search term
    for doc in matched_docs {
        let (num_pages, matched_page_nums, cropped_texts) = run_analysis(&doc, keyword);
        println!("File: {} \nNumber of pages: {}", doc, num_pages);
        println!("Search Term: ");

        for (idx, page) in matched_page_nums.iter().enumerate(){
            println!("   Page: {} \nExtracted Text: {}", page, cropped_texts[idx]);
        }
    }
    
    Ok(())
}
