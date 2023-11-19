use crate::error::FileOperationsError;

pub fn read_pdf(pdf_file: &str) -> Result<String, FileOperationsError> {
    // Open the PDF file
    let file: Vec<u8>= match std::fs::read(pdf_file) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::FileOpenError(pdf_file.to_string(), e))
    };
    
    // Extract text from the PDF
    let text: String = match pdf_extract::extract_text_from_mem(&file) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::FileReadError(pdf_file.to_string(), e))
    };

    Ok(text)
}

pub fn check_if_directory(file_or_directory: &str) -> bool {
    let split_dir: Vec<&str> = file_or_directory.split('/').collect::<Vec<&str>>();

    let last_item: &str = split_dir[split_dir.len()-1];
    let split_item: Vec<&str> = last_item.split('.').collect::<Vec<&str>>();

    if split_item.len() == 1 {
        return true
    }
    else {
        return false
    }
}

pub fn get_files_in_directory(directory: &str) -> Result<Vec<String>, FileOperationsError> {
    let mut files_list: Vec<String> = Vec::new();

    let file_paths = match std::fs::read_dir(directory) {
        Ok(s) => s,
        Err(e) => return Err(FileOperationsError::DirectoryReadError(directory.to_string(), e))
    };

    for file in file_paths {
        let file_name = file.unwrap().file_name().into_string().unwrap();
        files_list.push(format!("{}/{}",directory, file_name));
    }

    Ok(files_list)
}