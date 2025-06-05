#[cfg(test)]
mod tests {
    use crate::parser::detector::*;
    use crate::parser::extractor::*;
    use crate::parser::toml::*;
    use crate::parser::model::*;

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
}
