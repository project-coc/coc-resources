use crate::parser::detector::Style;
use regex::Regex;

pub struct Block {
    pub start_line: usize,
    pub end_line: usize,
    pub body: String,
}

pub fn extract_blocks(text: &str, _style: &Style) -> Vec<Block> {
    let re = Regex::new(r"(?m)^:::RESOURCE-START[\s\S]*?^:::RESOURCE-END$").unwrap();
    let mut res = Vec::new();
    for mat in re.find_iter(text) {
        let before = &text[..mat.start()];
        let start_line = bytecount::count(before.as_bytes(), b'\n') + 1;
        let block_text = mat.as_str();
        let body_start = block_text.find('\n').map(|i| i + 1).unwrap_or(0);
        let body_end = block_text.rfind('\n').unwrap_or(block_text.len());
        let body = &block_text[body_start..body_end];
        let end_line = start_line + bytecount::count(block_text.as_bytes(), b'\n');
        res.push(Block {
            start_line,
            end_line,
            body: body.to_string(),
        });
    }
    res
}
