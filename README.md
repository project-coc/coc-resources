# coc-resources

Rust implementation for parsing resource blocks embedded in source files.

This crate exposes a `parse_file` function which detects comment styles and
extracts resource metadata defined between `:::RESOURCE-START` and
`:::RESOURCE-END` delimiters. Parsed metadata is returned as `Resource`
structures.

Run tests with:

```bash
cargo test
```

## Command line usage

Build and run the CLI to parse a file:

```bash
cargo run -- parse example.rs
```
