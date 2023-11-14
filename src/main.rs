use std::fs::read as StdRead;
use std::path::Path as StdPath;
use tantivy::{Index, IndexWriter};
use tantivy::schema::{SchemaBuilder, TEXT, STORED, STRING};

fn read_pdf(pdf_file: &str) -> String {
    // Open the PDF file
    let file: Vec<u8> = StdRead(pdf_file).unwrap_or_else(|err| {
        panic!("{} - Unable to open PDF file", err);
    });
    
    // Extract text from the PDF
    let text: String = pdf_extract::extract_text_from_mem(&file).unwrap_or_else(|err| {
        panic!("{} - Unable to read contents of PDF file", err);
    });

    return text
}

fn create_or_open_index(index_path: &str) -> Index {
    let index_path: &StdPath = StdPath::new(index_path);
    let index: Index = if index_path.exists() {
        Index::open_in_dir(index_path).unwrap_or_else(|err| {
            panic!("{} - Unable to open existing folder to read index", err);
        })
    } else {
        let mut schema_builder: SchemaBuilder = SchemaBuilder::new();

        // Add fields to the schema
        let _ = schema_builder.add_text_field("content", TEXT | STORED);
        let _ = schema_builder.add_text_field("path", STRING | STORED);

        let index: Index = Index::builder()
            .schema(schema_builder.build())
            .create_in_dir(index_path).unwrap_or_else(|err| {
            panic!("{} - Failed to create directory for index writer", err);
        });

        // Set up the index writer
        let index_writer: IndexWriter = index
            .writer(50_000_000) // 50MB heap size for indexing
            .unwrap_or_else(|err| {
                panic!("{} - Failed to create index writer", err);
            });

        // Close the index writer
        drop(index_writer);

        index
    };

    return index
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the PDF file to be read
    let pdf_file: &str = "data/yolo.pdf";

    // Read text in PDF file
    let text:String = read_pdf(pdf_file);
    println!("{}", text.len());

    // Create or open the Tantivy index
    let index: Index = create_or_open_index("pdf_index");
    println!("{:#?}", index);

    Ok(())
}
