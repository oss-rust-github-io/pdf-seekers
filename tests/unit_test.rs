use pdf_seekers;

#[test]
fn valid_pdf_file_check() {
    let pdf_file: &str = "data/yolo.pdf";
    match pdf_seekers::file_operations::read_pdf(pdf_file) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Process should be able to read the file successfully")
    };
}

#[test]
fn invalid_directory_check() {
    let directory: &str = "invalid_dir";
    match pdf_seekers::file_operations::get_files_in_directory(directory) {
        Ok(_) => assert!(false, "Process should fail with FileOperationsError::DirectoryReadError"),
        Err(e) => match e {
            pdf_seekers::error::FileOperationsError::DirectoryReadError(_, _) => assert!(true),
            _ => assert!(false, "Process should fail with FileOperationsError::DirectoryReadError"),
        }
    };
}

#[test]
fn valid_directory_check() {
    let directory: &str = "data";
    match pdf_seekers::file_operations::get_files_in_directory(directory) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Process should be able to read the directory contents successfully")
    };
}