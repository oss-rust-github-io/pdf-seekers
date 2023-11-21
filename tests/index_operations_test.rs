use pdf_seekers;

#[test]
fn invalid_index_dir_create_check() {
    let index_path: &str = "X:/data/index_dir";
    match pdf_seekers::index_operations::create_or_open_index(index_path) {
        Ok(_) => assert!(false, "Process should fail with IndexingError::IndexDirectoryCreateError"),
        Err(e) => match e {
            pdf_seekers::error::IndexingError::IndexDirectoryCreateError(_, _) => assert!(true),
            _ => assert!(false, "Process should fail with IndexingError::IndexDirectoryCreateError"),
        }
    };
}

#[test]
fn tantivy_index_read_check() {
    let index_path: &str = "index_dir";
    match pdf_seekers::index_operations::create_or_open_index(index_path) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Process should be able to read the index_dir directory")
    };
}