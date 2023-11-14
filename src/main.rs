fn read_pdf(pdf_file: &str) -> String {
    // Open the PDF file
    let file: Vec<u8> = std::fs::read(pdf_file).unwrap_or_else(|err| {
        panic!("{} - Unable to open PDF file", err);
    });
    
    // Extract text from the PDF
    let text: String = pdf_extract::extract_text_from_mem(&file).unwrap_or_else(|err| {
        panic!("{} - Unable to read contents of PDF file", err);
    });

    return text
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the PDF file to be read
    let pdf_file: &str = "data/yolo.pdf";

    // Read text in PDF file
    let text:String = read_pdf(pdf_file);
    println!("{}", text);

    Ok(())
}
