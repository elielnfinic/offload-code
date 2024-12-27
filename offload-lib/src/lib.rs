use std::fs;

pub enum SupportedLanguages {
    RUST,
}

pub fn get_delete_dir_name(language: SupportedLanguages) -> String {
    match language {
        SupportedLanguages::RUST => "target".to_string(),
    }
}

pub fn get_language_type(files: Vec<String>) -> Vec<SupportedLanguages> {
    let mut supported_languages: Vec<SupportedLanguages> = Vec::new();

    let _ = files.iter().map(|file_name| {
        if file_name.to_lowercase().eq("cargo.toml") {
            supported_languages.push(SupportedLanguages::RUST);
        }
    });

    supported_languages
}

pub fn delete_folder(full_path : String) -> bool{
    match fs::remove_dir(full_path) {
        Ok(_) => true, 
        Err(_) => false
    }
}
