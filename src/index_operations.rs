use crate::file_operations::*;
use crate::error::IndexingError;
use tantivy::{Index, IndexWriter, Document};
use tantivy::schema::{SchemaBuilder, TEXT, STORED, STRING};

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

pub fn file_indexing(file_path: &str, index_path: &str) {
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