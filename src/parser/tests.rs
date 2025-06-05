#[cfg(test)]
mod tests {
    use crate::parser::detector::*;
    use crate::parser::extractor::*;
    use crate::parser::toml::*;
    use crate::parser::model::*;
    use crate::parse_file;
    use std::io::Write;

    #[test]
    fn detect_py() {
        let style = detect(std::path::Path::new("foo.py"));
        assert_eq!(style.line, Some("#"));
    }

    #[test]
    fn parse_block() {
        let text = r#":::RESOURCE-START
"名前" = "x"
"概要" = "y"
"タイプ" = "code"

["タグ"]
test = ""
:::RESOURCE-END"#;
        let blocks = extract_blocks(text, &Style { line: None, block_start: None, block_end: None });
        assert_eq!(blocks.len(), 1);
        let doc = parse(&blocks[0].body).unwrap();
        let res = Resource::from_toml(doc, std::path::Path::new("a.txt"), blocks[0].start_line, blocks[0].end_line);
        assert_eq!(res.name, "x");
    }

    #[test]
    fn line_comment_file() {
        let content = "// :::RESOURCE-START\n// \"名前\" = \"line\"\n// \"概要\" = \"g\"\n// \"タイプ\" = \"code\"\n// :::RESOURCE-END\n";
        let dir = std::env::temp_dir();
        let path = dir.join("sample.rs");
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        let res = parse_file(&path).unwrap();
        std::fs::remove_file(&path).ok();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "line");
    }

    #[test]
    fn block_comment_file() {
        let content = "/*\n:::RESOURCE-START\n\"名前\" = \"block\"\n\"概要\" = \"g\"\n\"タイプ\" = \"code\"\n:::RESOURCE-END\n*/";
        let dir = std::env::temp_dir();
        let path = dir.join("sample.c");
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        let res = parse_file(&path).unwrap();
        std::fs::remove_file(&path).ok();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "block");
    }
}
