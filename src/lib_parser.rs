use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrVec {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize)]
struct Library {
    words: HashMap<String, StringOrVec>,
}

pub fn load_library_from_toml(file_path: &str) -> io::Result<Vec<(String, String, String)>> {
    let contents = fs::read_to_string(file_path)?;
    let lib: Library = toml::from_str(&contents).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, e)
    })?;

    Ok(lib.words
        .into_iter()
        .map(|(k, v)| match v {
            StringOrVec::Single(s) => (k, s, String::new()),
            StringOrVec::Multiple(vec) => (
                k,
                vec.first().cloned().unwrap_or_default(),
                vec.get(1).cloned().unwrap_or_default()
            ),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_library() {
        let result = load_library_from_toml("lib/gojuon.toml");
        assert!(result.is_ok());
        
        if let Ok(lib) = result {
            assert!(!lib.is_empty());
            println!("Loaded {} entries", lib.len());
        }
    }
}