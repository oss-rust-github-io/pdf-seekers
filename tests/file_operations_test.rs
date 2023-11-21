use pdf_seekers;

#[test]
fn file_not_exist_check() {
    let pdf_file: &str = "invalid_file.pdf";
    match pdf_seekers::file_operations::read_pdf(pdf_file) {
        Ok(_) => assert!(false, "Process should fail with FileOperationsError::FileOpenError"),
        Err(e) => match e {
            pdf_seekers::error::FileOperationsError::FileOpenError(_, _) => assert!(true),
            _ => assert!(false, "Process should fail with FileOperationsError::FileOpenError"),
        }
    };
}

#[test]
fn invalid_pdf_file_check() {
    let pdf_file: &str = "data/invalid_file.pdf";
    match pdf_seekers::file_operations::read_pdf(pdf_file) {
        Ok(_) => assert!(false, "Process should fail with FileOperationsError::FileOpenError"),
        Err(e) => match e {
            pdf_seekers::error::FileOperationsError::FileReadError(_, _) => assert!(true),
            _ => assert!(false, "Process should fail with FileOperationsError::FileReadError"),
        }
    };
}

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

#[test]
fn invalid_search_file_check() {
    let pdf_file: String = String::from("data/invalid_file.pdf");
    match pdf_seekers::search_operations::run_analysis(&pdf_file, "convolutional") {
        Ok(_) => assert!(false, "Process should fail with FileOperationsError::PDFFileReadError"),
        Err(e) => match e {
            pdf_seekers::error::FileOperationsError::PDFFileReadError(_, _) => assert!(true),
            _ => assert!(false, "Process should fail with FileOperationsError::PDFFileReadError"),
        }
    };
}