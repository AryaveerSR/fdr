use std::{ffi::OsStr, path::PathBuf};

pub enum Pattern {
    Contains(String),
    Directory(String),
    Extension(String),
    DirectoryContains(String),
    All,
}

impl Pattern {
    // Takes relatvie paths.
    pub fn matches(&self, query: &PathBuf) -> bool {
        match self {
            Pattern::Contains(string) => query
                .file_name()
                .unwrap()
                .to_string_lossy()
                .contains(string),
            Pattern::Directory(string) => query.starts_with(string),
            Pattern::Extension(string) => {
                string
                    == &query
                        .extension()
                        .unwrap_or(OsStr::new(""))
                        .to_string_lossy()
                        .to_string()
            }
            Pattern::DirectoryContains(string) => query.display().to_string().contains(string),
            Pattern::All => true,
        }
    }

    pub fn parse(pattern_vec: &Vec<String>) -> Vec<Self> {
        if pattern_vec.is_empty() {
            return vec![Pattern::All];
        }

        let mut patterns: Vec<Pattern> = vec![];

        for pattern in pattern_vec {
            if pattern.starts_with("/") && pattern.chars().nth(1).is_some() {
                if pattern.chars().nth(1) == Some('*') {
                    patterns.push(Pattern::DirectoryContains(pattern[2..].to_string()));
                } else {
                    patterns.push(Pattern::Directory(pattern[1..].to_string()));
                }
            } else if pattern.starts_with("*") {
                if pattern.chars().nth(1).is_some() {
                    if pattern[1..].starts_with(".") {
                        patterns.push(Pattern::Extension(pattern[2..].to_string()))
                    } else {
                        patterns.push(Pattern::Contains(pattern[1..].to_string()))
                    }
                } else {
                    return vec![Pattern::All];
                }
            } else if pattern.starts_with(".") {
                patterns.push(Pattern::Extension(pattern[1..].to_string()))
            } else {
                patterns.push(Pattern::Contains(pattern.to_string()));
            }
        }

        patterns
    }
}
