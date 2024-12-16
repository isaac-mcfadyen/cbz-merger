# cbz-merger

A small application to merge a collection of `.cbz` (comic book archive) files into a single file. Useful when books are published as one-per-chapter but you want to transfer and read them all as one on an e-reader/application.

## Install

This tool is written in Rust. To install it, first install [Rust](https://www.rust-lang.org/tools/install), then run the following command:

```
cargo install --locked --git https://github.com/isaac-mcfadyen/cbz-merger
```

## Usage

After install, merge a collection of books by using the following command. Substitute `<NAME>` with the desired `.cbz` output file and `<INPUT_FILES>` with the files to merge (optionally using a glob to merge all files in a directory):

```sh
cbz-merger -o <NAME>.cbz <INPUT_FILES>...
```

Full help can be found by running `cbz-merger --help`.
