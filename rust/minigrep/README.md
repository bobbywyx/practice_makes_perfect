# minigrep in Rust

## Description

This mini grep tool is implemented in Rust. Inspired by [rust course](https://course.rs/about-book.html).

## Usage

This tool take 3 command parameters currently, which are:
- `query`: the query to search for
- `filename`: the file to search in
- `case_sensitive`: whether the search is case-sensitive, "i" for insensitive, default is case-sensitive

example:
```bash
cargo run -- to poem.txt

cargo run -- to poem.txt i # case insensitive
```

## Dependencies

Tested and built under `rustc v1.72.1`