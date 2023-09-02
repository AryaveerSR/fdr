<div style="text-align: center;" align="center">

# `fdr`

## A file search tool written in rust!

[![MIT](https://img.shields.io/crates/l/bitvec.svg?style=for-the-badge)](LICENSE)

</div>

## About

This is a basic file search tool I wrote as an exercise. It supports a small subset of features and has a small, pretty neat source.

## Arguments

If no pattern is passed, it lists all files in the directory.
See [`pattern.rs`](src/pattern.rs) for all the rules.

- Free arguments: Pattern(s) for matching files. They can be:

  - `/` indicates the subdirectories to search:
    - `/*tar` will match all files having a parent directory containing `tar` in its name.
    - `/target` will only match files with `target` as the parent directory.
  - `.` to indicate the file extension:
    - `*.exe` matches all files ending in `.exe`
  - `*` matches everything.

  > You can also pass multiple of these and only list files which match them all.

- `--help` for help
- `--folders` / `-f` for listing folders.
- `--root` / `-r` for specifying the directory to search.
- `--depth` / `-d` for specifying the maximum depth to search.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
