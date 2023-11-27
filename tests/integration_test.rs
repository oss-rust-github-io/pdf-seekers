use pdf_seekers;

#[test]
fn indexing_check() {
    let file_or_directory: &str = "data/yolo.pdf";
    
    match pdf_seekers::indexing_contents(file_or_directory, None, None) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Indexing process should execute successfully")
    };
}

#[test]
fn keyword_search_check() {
    let file_or_directory: &str = "data";
    let search_term: String = String::from("convolutional");

    match pdf_seekers::search_term_in_file(file_or_directory, search_term, None, None) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Searching process should execute successfully")
    };
}