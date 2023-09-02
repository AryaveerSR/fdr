//! Code related to interacting with the commandline.

use gumdrop::Options;
use std::{env, fs, path::PathBuf};

// The commandline arguments accepted by the tool.
#[derive(Debug, Options)]
pub struct AppOptions {
    #[options(free)]
    free: Vec<String>,

    #[options(help = "Shows the not-so-helpful help message.")]
    help: bool,

    #[options(help = "The directory to search. Defaults to the currently open one.")]
    root: Option<String>,

    #[options(help = "Maximum depth to search.")]
    pub depth: Option<u8>,

    #[options(help = "If it should include folders in the results.")]
    pub folders: bool,
}

// Some utility functions to make code tidy elsewhere.
impl AppOptions {
    pub fn parse() -> Self {
        Self::parse_args_default_or_exit()
    }

    pub fn has_free_args(&self) -> bool {
        !self.free.is_empty()
    }

    pub fn get_free(&self) -> &Vec<String> {
        &self.free
    }

    pub fn path(&self) -> PathBuf {
        match &self.root {
            Some(dir) => fs::canonicalize(dir).expect("Get root dir path from CLI argument."),
            // Use the currently open directory if no path is specified.
            None => env::current_dir().expect("Get current dir as root dir."),
        }
    }
}

pub struct Entry {
    content: String,
    is_file: bool,
}

impl Entry {
    pub fn new(content: String, is_file: bool) -> Self {
        Entry { content, is_file }
    }
}

// Glyphs for reference for future-me : ─│╭╮╰╯┼┴┬┤├
/// Function to draw a table displaying entries as a table with serial numbers.
pub fn draw_file_table(files: &Vec<Entry>) {
    // Getting the maximum number of digits the serial numbers will occupy.
    // Used to calculate the width of the serial column.
    let max_sr_len = (0..)
        .take_while(|i| 10u32.pow(*i) <= ((files).len() + 1) as u32)
        .count();

    // Getting maximum number of characters the path will occupy.
    // Again, used to calculate the width of files column.
    let max_path_len = {
        let mut x = 0;

        for entry in files {
            let len = entry.content.len();
            x = x.max(len);
        }

        x
    };

    // Calculate the padding for serial numbers.
    // The `2` is the length of the string "Sr" itself, which would be the longest string if
    // the number of files are less than 10.
    let sr_padding = 2.max(max_sr_len);

    // Similar to `sr_padding`. `5` here is from length of "Files".
    let file_padding = 5.max(max_path_len);

    // Genuinely ugly drawing code.
    println!(
        "╭{}┬{}╮",
        "─".repeat(sr_padding + 2), // The `+2`s account for extra space padding on left and right.
        "─".repeat(file_padding + 2),
    );
    // Neat `format!` syntax for easy right-side padding.
    println!("│ {:<sr_padding$} │ {:<file_padding$} │", "Sr", "Files");
    println!(
        "├─{}─┼─{}─┤",
        "─".repeat(sr_padding),
        "─".repeat(file_padding),
    );
    // Loop over all entries.
    for (i, entry) in files.iter().enumerate() {
        // Display the serial number.
        print!("│ {:<sr_padding$} │ ", i + 1);
        // ..check the entry type and color ahoy!
        if entry.is_file {
            print!("\x1b[92m{:<file_padding$}\x1b[m", entry.content);
        } else {
            print!("\x1b[93m{:<file_padding$}\x1b[m", entry.content);
        }
        println!(" │");
    }

    // Finally its over.
    println!(
        "╰─{}─┴─{}─╯",
        "─".repeat(sr_padding),
        "─".repeat(file_padding),
    );

    print!("\x1b[m");
}
