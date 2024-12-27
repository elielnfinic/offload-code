use std::fs;

use walkdir::WalkDir;

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

pub fn delete_folder(full_path: String) -> bool {
    fs::remove_dir(full_path).is_ok()
}

#[derive(Debug, Clone)]
pub enum DirSizeUnit {
    Byte,
}

#[derive(Debug, Clone)]
pub struct DirSize {
    pub size: u64,
    pub unit: DirSizeUnit,
}

pub fn get_dir_size(full_path: String) -> DirSize {
    let mut total_dir_size = 0;

    for entry in WalkDir::new(full_path.as_str())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        total_dir_size += filesize::file_real_size(entry.path()).unwrap_or_default();
    }

    DirSize {
        size: total_dir_size,
        unit: DirSizeUnit::Byte,
    }
}

#[cfg(test)]
mod test {
    use crate::get_dir_size;

    #[test]
    fn evaluate_file_size() {
        let the_size = get_dir_size("Cargo.toml".to_string());
        assert!(the_size.size > 0);
    }
}
