use std::path::Path;

#[derive(Debug, Clone)]
pub struct Style {
    pub line: Option<&'static str>,
    pub block_start: Option<&'static str>,
    pub block_end: Option<&'static str>,
}

static DEFAULT_TABLE: &[(&str, Style)] = &[
    (
        ".rs",
        Style {
            line: Some("//"),
            block_start: Some("/*"),
            block_end: Some("*/"),
        },
    ),
    (
        ".c",
        Style {
            line: Some("//"),
            block_start: Some("/*"),
            block_end: Some("*/"),
        },
    ),
    (
        ".h",
        Style {
            line: Some("//"),
            block_start: Some("/*"),
            block_end: Some("*/"),
        },
    ),
    (
        ".cpp",
        Style {
            line: Some("//"),
            block_start: Some("/*"),
            block_end: Some("*/"),
        },
    ),
    (
        ".ts",
        Style {
            line: Some("//"),
            block_start: Some("/*"),
            block_end: Some("*/"),
        },
    ),
    (
        ".js",
        Style {
            line: Some("//"),
            block_start: Some("/*"),
            block_end: Some("*/"),
        },
    ),
    (
        ".py",
        Style {
            line: Some("#"),
            block_start: Some("\"\"\""),
            block_end: Some("\"\"\""),
        },
    ),
    (
        ".md",
        Style {
            line: None,
            block_start: None,
            block_end: None,
        },
    ),
    (
        ".txt",
        Style {
            line: None,
            block_start: None,
            block_end: None,
        },
    ),
    (
        ".html",
        Style {
            line: Some("//"),
            block_start: Some("<!--"),
            block_end: Some("-->"),
        },
    ),
    (
        ".vue",
        Style {
            line: Some("//"),
            block_start: Some("<!--"),
            block_end: Some("-->"),
        },
    ),
];

pub fn detect(path: &Path) -> Style {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let ext = format!(".{}", ext);
    for (key, style) in DEFAULT_TABLE {
        if *key == ext {
            return style.clone();
        }
    }
    Style {
        line: Some("//"),
        block_start: Some("/*"),
        block_end: Some("*/"),
    }
}
