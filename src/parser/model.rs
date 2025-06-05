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
    pub tags: Vec<Tag>,
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
        let tags = doc
            .get("タグ")
            .and_then(|item| item.as_table())
            .map(|tbl| {
                tbl.iter()
                    .map(|(k, v)| {
                        let desc = v
                            .as_str()
                            .map(|s| s.to_string())
                            .and_then(|s| if s.is_empty() { None } else { Some(s) });
                        Tag {
                            tag: k.to_string(),
                            description: desc,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        Resource {
            path: path.to_path_buf(),
            line_start: start,
            line_end: end,
            name,
            summary,
            r#type,
            tags,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
    pub description: Option<String>,
}
