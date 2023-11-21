use pdf_seekers;

#[test]
fn indexing_check() {
    let file_or_directory: &str = "data/yolo.pdf";
    let index_path: &str = "index_dir";
    match pdf_seekers::indexing_contents(file_or_directory, index_path) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Indexing process should execute successfully")
    };
}

#[test]
fn keyword_search_check() {
    let file_or_directory: &str = "data";
    let index_path: &str = "index_dir";
    let search_term: String = String::from("convolutional");
    match pdf_seekers::search_term_in_file(file_or_directory, index_path, &search_term) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Searching process should execute successfully")
    };
}