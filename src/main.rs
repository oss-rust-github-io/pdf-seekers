use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{Index, IndexWriter, Document};
use tantivy::schema::{SchemaBuilder, TEXT, STORED, STRING};

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

fn create_or_open_index(index_path: &str) -> Index {
    std::fs::create_dir_all(index_path).unwrap_or_else(|err| {
        panic!("{} - Unable to create folder to store index", err);
    });

    let dir_content = std::fs::read_dir(index_path).unwrap_or_else(|err| {
        panic!("{} - Unable to open folder to read contents", err);
    });

    let dir_check: bool = dir_content.count() == 0;

    let index: Index = if !dir_check {
        Index::open_in_dir(index_path).unwrap_or_else(|err| {
            panic!("{} - Unable to read index in existing folder", err);
        })
    } else {
        let mut schema_builder: SchemaBuilder = SchemaBuilder::new();

        // Add fields to the schema
        schema_builder.add_text_field("content", TEXT | STORED);
        schema_builder.add_text_field("path", STRING | STORED);

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

fn parse_and_index_pdf(pdf_file: &str, pdf_text: String, index: &Index) {
    // Create a Tantivy index writer
    let mut index_writer = index
        .writer_with_num_threads(1, 40_000_000)
        .unwrap_or_else(|err| {
            panic!("{} - Failed to create index writer", err);
        });

    // Create a Tantivy document
    let mut doc = Document::default();

    // Add PDF path to the document
    doc.add_text(index.schema().get_field("content").unwrap(), &pdf_text);
    doc.add_text(
        index.schema().get_field("path").unwrap(),
        &pdf_file.to_string(),
    );

    // Add the document to the index
    index_writer.add_document(doc).unwrap_or_else(|err| {
        panic!("{} - Failed to add PDF content to the index", err);
    });

    // Commit changes to the index
    index_writer.commit().unwrap_or_else(|err| {
        panic!("{} - Failed to commit changes to the index", err);
    });
}

fn search_keyword(index: &Index, query_str: &str) {
    let indexer = index.reader().unwrap_or_else(|err| {
        panic!("{} - Failed to create reader object for Indexer", err);
    });
    let searcher = indexer.searcher();
    let schema = index.schema();
    let query_parser = QueryParser::for_index(&index, vec![schema.get_field("content").unwrap()]);

    // Parse the query string
    let query = query_parser.parse_query(query_str).unwrap_or_else(|err| {
        panic!("{} - Failed to parse query for PDF document", err);
    });

    // Search the index
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap_or_else(|err| {
        panic!("{} - Failed to search keyword in PDF document", err);
    });
    println!("{:#?}", top_docs);

    // Display search results
    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap_or_else(|err| {
            panic!("{} - Failed to retrieve PDF document", err);
        });
        let path_field = schema.get_field("path").unwrap();
        let path_value = retrieved_doc
            .get_first(path_field)
            .and_then(|v| v.as_text())
            .unwrap_or_default();

        println!("Score: {:.2}, Path: {}", score, path_value);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the PDF file to be read
    let pdf_file: &str = "data/fast_rcnn.pdf";

    // Read text in PDF file
    let pdf_text:String = read_pdf(pdf_file);

    // Create or open the Tantivy index
    let index: Index = create_or_open_index("pdf_index");

    // Parse PDF and index content
    parse_and_index_pdf(pdf_file, pdf_text, &index);

    // Search for a term in the indexed PDFs
    search_keyword(&index, "convolutional");

    Ok(())
}
