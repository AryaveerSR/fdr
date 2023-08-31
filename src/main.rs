use anyhow::Result;
use gumdrop::Options;
use std::{env, fs, path::PathBuf};

#[derive(Debug, Options)]
struct AppOptions {
    #[options(free)]
    free: Vec<String>,

    #[options(help = "Shows the not-so-helpful help message.")]
    help: bool,

    #[options(help = "The directory to search. Defaults to the currently open one.")]
    dir: Option<String>,
}

fn leven(a: &str, b: &str) -> u8 {
    if a.is_empty() {
        return b.len() as u8;
    }

    if b.is_empty() {
        return a.len() as u8;
    }

    // `.expect()` is not nessessary on `.nth()` as we already checked for if the strings are empty
    if a.chars().nth(0).unwrap().to_ascii_lowercase()
        == b.chars().nth(0).unwrap().to_ascii_lowercase()
    {
        return leven(&a[1..], &b[1..]);
    }

    let a_tailed = leven(&a[1..], b);
    let b_tailed = leven(a, &b[1..]);
    let a_b_tailed = leven(&a[1..], &b[1..]);

    a_tailed.min(b_tailed).min(a_b_tailed) + 1
}

fn recursive_match(search: &str, path: PathBuf) -> Result<Vec<(u8, PathBuf)>> {
    let mut matches: Vec<(u8, PathBuf)> = vec![];
    for i in fs::read_dir(path)? {
        let entry = i?;
        if entry.metadata()?.is_dir() {
            matches.append(&mut recursive_match(search, entry.path())?);
        } else {
            matches.push((
                leven(search, entry.file_name().to_str().unwrap()),
                entry.path(),
            ))
        }
    }
    Ok(matches)
}

fn main() -> Result<()> {
    let opts: AppOptions = AppOptions::parse_args_default_or_exit();
    let dir = match opts.dir {
        Some(dir) => fs::canonicalize(dir).expect("Get root dir path from CLI argument."),
        None => env::current_dir().expect("Get current dir as root dir."),
    };

    if opts.free.is_empty() {
        panic!("No Argument supplied"); //todo! return a dir listing instead ??
    }

    let search = opts.free[0].as_str();
    let mut results: Vec<(u8, PathBuf)> = recursive_match(search, dir)?;

    results.sort_by(|(a, _), (b, _)| a.cmp(b));

    dbg!(results);

    Ok(())
}
