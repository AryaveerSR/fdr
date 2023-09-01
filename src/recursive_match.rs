use anyhow::{Context, Result};
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

/// A struct for storing user-defined file-matching options, that are to be passed to `recursive_match` that
/// turns it into a closure to evaluate entries.
pub struct MatchOptions {
    /// Maximum depth of recursion.
    pub depth: u8,
    /// If to match every single file, used if no search argument is defined.
    pub match_all: bool,
}

impl MatchOptions {
    // Default constants for options.
    const DEFAULT_DEPTH: u8 = 3;

    pub fn new(depth: Option<u8>, match_all: bool) -> Self {
        Self {
            depth: depth.unwrap_or(Self::DEFAULT_DEPTH),
            match_all,
        }
    }
}

/// Function to recursively match files in a directory (and its subdirectories) using user defined parameters passed
/// as `opts` of type `MatchOptions`.
pub fn recursive_match(search: &str, path: &PathBuf, opts: &MatchOptions) -> Result<Vec<PathBuf>> {
    // Use the `opts` to create an closure that checks if a file satisfies the conditions..
    let file_match = move |file: &DirEntry| {
        if opts.match_all {
            true
        } else {
            file.file_name()
                .to_ascii_lowercase()
                .to_str()
                .unwrap()
                .contains(search)
        }
    };
    // ..and another closure to decide whether to traverse a subdirectory provided depth isn't exceeded.
    let folder_match = |_folder: &DirEntry| true;

    // Run the internal recursive function and return the results.
    in_recursive_match(&file_match, &folder_match, path, opts.depth)
}

/// The actual recursive function to loop over all entries that satisfy the `file_match` and `folder_match` closures.
///
/// `depth` is the maximum depth of recursion.
fn in_recursive_match(
    file_match: &impl Fn(&DirEntry) -> bool,
    folder_match: &impl Fn(&DirEntry) -> bool,
    path: &PathBuf,
    depth: u8,
) -> Result<Vec<PathBuf>> {
    let mut matches: Vec<PathBuf> = vec![];

    // Loop through every entry in the directory.
    for i in fs::read_dir(path).context("Cannot read directory.")? {
        let entry = i?;

        // If its another directory, check if the depth isn't exceeded and that it matches the `folder_match`..
        if entry.metadata()?.is_dir() && depth != 0 && folder_match(&entry) {
            // ..and run itself for the subdirectory (decreasing depth by 1), and merge all entries into the `matches` vec.
            matches.append(&mut in_recursive_match(
                file_match,
                folder_match,
                &entry.path(),
                depth - 1,
            )?);

        // Or if its a file, run the `file_match` on it..
        } else if file_match(&entry) {
            // ..and push to `matches` if it satisfies.
            matches.push(entry.path())
        }
    }

    Ok(matches)
}
