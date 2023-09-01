use anyhow::Result;
use fdr::{
    cli::{draw_file_table, AppOptions},
    levenstein_distance,
    recursive_match::{recursive_match, MatchOptions},
};
use std::path::PathBuf;

fn main() -> Result<()> {
    // Parse arguments.
    let args: AppOptions = AppOptions::parse();
    if !args.has_free_args() {
        panic!("No search argument supplied"); //todo! return a dir listing instead ??
    }

    let dir = args.path();
    let search = args.get_free().to_ascii_lowercase();
    let opts = MatchOptions::new(args.depth());

    // Find all matches satisfying `opts` MatchOptions.
    let mut result_paths: Vec<PathBuf> = recursive_match(search.as_ref(), &dir, &opts)?;

    // Sort according to levenstein distance (probably not the best way, but sounds cool).
    result_paths.sort_by_cached_key(|path| {
        levenstein_distance(&search, path.file_name().unwrap().to_str().unwrap())
    });

    // Make paths relative to root directory, and then turn them into strings..
    let result = result_paths
        .iter()
        .map(|path| path.strip_prefix(&dir).unwrap().display().to_string())
        .collect::<Vec<String>>();

    // ..and finally draw them.
    draw_file_table(&result);
    Ok(())
}
