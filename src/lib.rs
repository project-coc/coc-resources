pub mod parser;
use anyhow::Result;
use std::fs;
use std::path::Path;
use parser::{detector, extractor, toml as toml_parser, model::Resource};

pub fn parse_file(path: &Path) -> Result<Vec<Resource>> {
    let style = detector::detect(path);
    let text = fs::read_to_string(path)?;
    let mut resources = Vec::new();
    for block in extractor::extract_blocks(&text, &style) {
        let doc = toml_parser::parse(&block.body)?;
        let res = Resource::from_toml(doc, path, block.start_line, block.end_line);
        resources.push(res);
    }
    Ok(resources)
}
