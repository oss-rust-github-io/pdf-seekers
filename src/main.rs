use pdf_seekers::{
    read_pdf, 
    create_or_open_index, 
    parse_and_index_pdf, 
    search_keyword, 
    run_analysis
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
