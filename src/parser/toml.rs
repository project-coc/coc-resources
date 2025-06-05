use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
use toml_edit::{Document, Item};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("missing required key: {0}")]
    MissingKey(&'static str),
    #[error("invalid enum value in `タイプ`")]
    BadType,
    #[error(transparent)]
    Toml(#[from] toml_edit::TomlError),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResType {
    #[serde(rename = "code")] Code,
    #[serde(rename = "doc")] Doc,
    #[serde(rename = "asset")] Asset,
    #[serde(rename = "data")] Data,
    #[serde(rename = "test")] Test,
    #[serde(rename = "other")] Other,
}

pub fn parse(raw: &str) -> Result<Document, ParseError> {
    let doc = Document::from_str(raw)?;
    validate_required(&doc)?;
    Ok(doc)
}

fn lookup<'a>(doc: &'a Document, key: &str) -> Option<&'a Item> {
    doc.as_table().get(key)
}

pub fn validate_required(doc: &Document) -> Result<(), ParseError> {
    for key in ["名前", "概要", "タイプ"] {
        if lookup(doc, key).is_none() {
            return Err(ParseError::MissingKey(key));
        }
    }
    if let Some(item) = lookup(doc, "タイプ") {
        let val = item.as_str().unwrap_or("");
        match val {
            "code" | "doc" | "asset" | "data" | "test" | "other" => {}
            _ => return Err(ParseError::BadType),
        }
    }
    Ok(())
}
