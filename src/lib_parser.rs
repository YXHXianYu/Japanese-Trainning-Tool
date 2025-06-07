use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Deserialize)]
struct Library {
    words: HashMap<String, String>,
}

pub fn load_library_from_csv(file_path: &str) -> io::Result<Vec<(String, String)>> {
    let contents = fs::read_to_string(file_path)?;
    let lib: Library = toml::from_str(&contents).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, e)
    })?;

    Ok(lib.words
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_library() {
        let result = load_library_from_csv("data/library.toml");
        assert!(result.is_ok());
        
        if let Ok(lib) = result {
            assert!(!lib.is_empty());
            println!("Loaded {} entries", lib.len());
        }
    }
}