use serde::{Serialize, Deserialize};
use toml_edit::Document;
use super::toml::ResType;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub path: PathBuf,
    pub line_start: usize,
    pub line_end: usize,
    pub name: String,
    pub summary: String,
    pub r#type: ResType,
}

impl Resource {
    pub fn from_toml(doc: Document, path: &std::path::Path, start: usize, end: usize) -> Self {
        let name = doc["名前"].as_str().unwrap_or("").to_string();
        let summary = doc["概要"].as_str().unwrap_or("").to_string();
        let t = doc["タイプ"].as_str().unwrap_or("other");
        let r#type = match t {
            "code" => ResType::Code,
            "doc" => ResType::Doc,
            "asset" => ResType::Asset,
            "data" => ResType::Data,
            "test" => ResType::Test,
            _ => ResType::Other,
        };
        Resource {
            path: path.to_path_buf(),
            line_start: start,
            line_end: end,
            name,
            summary,
            r#type,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
    pub description: Option<String>,
}
