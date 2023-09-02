use anyhow::Result;
use fdr::{
    cli::{draw_file_table, AppOptions, Entry},
    pattern::Pattern,
    recursive_match::{recursive_match, MatchOptions},
};
use std::path::PathBuf;

fn main() -> Result<()> {
    // Parse arguments.
    let args: AppOptions = AppOptions::parse();

    let dir = args.path();
    let pattern = Pattern::parse(args.get_free());
    let opts = MatchOptions::new(args.depth, args.folders);

    // Find all matches satisfying `opts` MatchOptions.
    let result_paths: Vec<PathBuf> = recursive_match(&pattern, &dir, opts)?;

    // Make paths relative to root directory, and then turn them into strings..
    let result = result_paths
        .iter()
        .map(|path| {
            Entry::new(
                path.strip_prefix(&dir).unwrap().display().to_string(),
                path.is_file(),
            )
        })
        .collect::<Vec<Entry>>();

    // ..and finally draw them.
    draw_file_table(&result);
    Ok(())
}
