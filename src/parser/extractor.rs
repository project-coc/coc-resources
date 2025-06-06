use crate::parser::detector::Style;
use regex::Regex;

pub struct Block {
    pub start_line: usize,
    pub end_line: usize,
    pub body: String,
}

pub fn extract_blocks(text: &str, style: &Style) -> Vec<Block> {
    // Normalize line endings so that regex patterns relying on `\n` work
    // correctly for files with Windows-style CRLF endings.
    let normalized_text = text.replace("\r\n", "\n").replace('\r', "\n");
    let text = normalized_text.as_str();

    let mut res = Vec::new();

    if style.line.is_none() && style.block_start.is_none() {
        // plain blocks without any comment markers
        let bare_re = Regex::new(r"(?m)^:::RESOURCE-START[\s\S]*?^:::RESOURCE-END$").unwrap();
        for mat in bare_re.find_iter(text) {
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
    }

    if let Some(prefix) = style.line {
        let pattern = format!(
            r"(?m)^{0}\s*:::RESOURCE-START[\s\S]*?^{0}\s*:::RESOURCE-END$",
            regex::escape(prefix)
        );
        let re = Regex::new(&pattern).unwrap();
        for mat in re.find_iter(text) {
            let before = &text[..mat.start()];
            let start_line = bytecount::count(before.as_bytes(), b'\n') + 1;
            let block_text = mat.as_str();
            let body_start = block_text.find('\n').map(|i| i + 1).unwrap_or(0);
            let body_end = block_text.rfind('\n').unwrap_or(block_text.len());
            let raw_body = &block_text[body_start..body_end];
            let body: String = raw_body
                .lines()
                .map(|l| {
                    let mut line = l;
                    if line.starts_with(prefix) {
                        line = &line[prefix.len()..];
                        if line.starts_with(' ') {
                            line = &line[1..];
                        }
                    }
                    line
                })
                .collect::<Vec<_>>()
                .join("\n");
            let end_line = start_line + bytecount::count(block_text.as_bytes(), b'\n');
            res.push(Block {
                start_line,
                end_line,
                body,
            });
        }
    }

    if let (Some(start), Some(end)) = (style.block_start, style.block_end) {
        let comment_pat = format!("{}[\\s\\S]*?{}", regex::escape(start), regex::escape(end));
        let comment_re = Regex::new(&comment_pat).unwrap();
        let delim_re = Regex::new(r":::RESOURCE-START[\s\S]*?:::RESOURCE-END").unwrap();
        for comment in comment_re.find_iter(text) {
            let comment_inner = &text[comment.start() + start.len()..comment.end() - end.len()];
            for mat in delim_re.find_iter(comment_inner) {
                let abs_start = comment.start() + start.len() + mat.start();
                let before = &text[..abs_start];
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
        }
    }

    res
}
