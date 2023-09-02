//! Module containing pattern matching implementation.

use std::{ffi::OsStr, path::PathBuf};

/// An enum for different types of supported patterns.
pub enum Pattern {
    /// If the query contains the value.
    Contains(String),
    /// If the query is in the directory.
    Directory(String),
    /// If the query has the extension.
    Extension(String),
    /// If the query's directory contains a keyword
    DirectoryContains(String),
    /// EVERYTHING!!!!
    All,
}

impl Pattern {
    /// Function that matches a `query` with its pattern.
    pub fn matches(&self, query: &PathBuf) -> bool {
        match self {
            Pattern::Extension(string) => {
                string
                    == &query
                        .extension()
                        .unwrap_or(OsStr::new(""))
                        .to_string_lossy()
                        .to_string()
            }
            Pattern::Contains(string) => query
                .file_name()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .contains(string),
            Pattern::Directory(string) => query.starts_with(string),
            Pattern::DirectoryContains(string) => query.display().to_string().contains(string),
            Pattern::All => true,
        }
    }

    /// Function that parses a vec of pattern strings into a vec of `Pattern`
    pub fn parse(pattern_vec: &Vec<String>) -> Vec<Self> {
        // Empty basically turns it into a `dir` command.
        if pattern_vec.is_empty() {
            return vec![Pattern::All];
        }

        let mut patterns: Vec<Pattern> = vec![];

        for pattern in pattern_vec {
            // If it has a '/', it involves directories.
            if pattern.starts_with("/") && pattern.chars().nth(1).is_some() {
                if pattern.chars().nth(1) == Some('*') {
                    // Patterns like "/*tar" should match anything in the directories "tar", "target", etc.
                    patterns.push(Pattern::DirectoryContains(pattern[2..].to_string()));
                } else {
                    // Patterns like "/target" should match anything only in the "target" directory.
                    patterns.push(Pattern::Directory(pattern[1..].to_string()));
                }
            } else if pattern.starts_with("*") {
                if pattern.chars().nth(1).is_some() {
                    if pattern[1..].starts_with(".") {
                        // Patterns like "*.exe" should match all files having ".exe" extension.
                        // Files with a extension, say ".execute", will not be matched.
                        patterns.push(Pattern::Extension(pattern[2..].to_string()))
                    } else {
                        // Patterns like "*hello" should match "hello", "hello.exe", etc.
                        // It also matches the entire name, so "*p.exe" would match "help.exe" file.
                        patterns.push(Pattern::Contains(pattern[1..].to_string()))
                    }
                }
            } else if pattern.starts_with(".") {
                // Patterns like ".exe" should do the same thing as "*.exe", that is match any files
                // with ".exe" extension. Like "*.exe", it won't match a file with, say ".execute" extension.
                patterns.push(Pattern::Extension(pattern[1..].to_string()))
            } else {
                // The fallback case is same as "*something" pattern, that is matching any file
                // with the pattern string in its name.
                patterns.push(Pattern::Contains(pattern.to_string()));
            }
        }

        patterns
    }
}
