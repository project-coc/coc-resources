# coc-resource Specification v1.3

This document summarizes the implementation requirements for the `coc-resource` toolchain, based entirely on the "coc-resource 仕様書 v1.3" content provided in the prompt.

## Terminology

| Term | Description |
|------|-------------|
| **リソースブロック** | TOML-compatible metadata embedded in a file between `:::RESOURCE-START` and `:::RESOURCE-END`. |
| **coc r** | `coc` toolchain subcommand: invoked as `coc r <sub>` for CLI operations. |
| **タグマスタ** | Table that stores unique tags, their hierarchy and aliases. |

## Goals and Non-goals

| Goals | Non-goals |
|------|-----------|
| Extract metadata including line positions. | Source AST analysis. |
| CRUD operations in SQLite via rusqlite with automatic schema migration. | Support for other RDBMS. |
| CLI subcommands `import`, `query`, `export`, `validate`, `tag`. | GUI client. |
| Tag database supporting alias, parent tree and usage stats. | Advanced ontology (RDF/OWL). |
| Interfaces via gRPC and JSON-RPC. | Web UI. |

## Architecture Overview

```
+--------------------+          +----------------------+          +-------------------+
|  coc r (CLI)       | <grpc>   | Resource Service     |  rusqlite|  SQLite DB        |
|  └─ subcommands    |────────► |  (tonic gRPC server) |────────► |  (resource, tag)  |
+--------------------+          +----------------------+          +-------------------+
                                        ▲
                                        │ JSON-RPC (optional)
                                        ▼
                               +----------------------+
                               |  Other AI agents     |
                               +----------------------+
```

## Resource Block Syntax

A block consists of required delimiters at the beginning of a line:

```
:::RESOURCE-START
...TOML body...
:::RESOURCE-END
```

- TOML body must follow standard TOML syntax (no comments).
- Required keys: `名前`, `概要`, `タイプ`.
- `タイプ` must be one of: `code`, `doc`, `asset`, `data`, `test`, `other`.
- `[メタデータ]` and `[タグ]` sections are optional.
- `[タグ]` entries use tag names as keys, with empty string or description as values.

### ABNF (excerpt)

```
block      = start delim *1(TAB / SP) newline toml_body end delim newline
start delim= ":::RESOURCE-START"
end delim  = ":::RESOURCE-END"
```

### Comment Style Detection

| Extension               | Line comment | Block start | Block end |
|-------------------------|--------------|-------------|-----------|
| `.rs`, `.c`, `.h`, `.cpp`, `.ts`, `.js` | `//` | `/*` | `*/` |
| `.py`                   | `#` | `"""` | `"""` |
| `.md`, `.txt`           | *none* | *none* | *none* |
| `.html`, `.vue`         | `//` | `<!--` | `-->` |

The parser supports both line and block comment forms based on file extension.

## Parser Modules (Rust)

```
crate::parser
├── detector.rs      # Comment style detection
├── extractor.rs     # Regex search for start/end delimiters
├── toml.rs          # toml_edit parsing and validation
├── model.rs         # Resource, Tag structs
└── tests.rs
```

Pseudo-code for `parse_file`:

```rust
fn parse_file(path: &Path) -> Result<Vec<Resource>> {
    let style = detector::detect(path);
    let text  = fs::read_to_string(path)?;
    for (start_idx,end_idx,raw) in extractor::extract_blocks(&text, &style) {
        let doc = toml_edit::Document::from_str(&raw)?;
        validate_required(&doc)?;
        let res = Resource::from_toml(doc, path, start_idx, end_idx);
        resources.push(res);
    }
    Ok(resources)
}
```

Regular expression used in `extractor`:

```
(?m)^:::RESOURCE-START[\s\S]*?^:::RESOURCE-END$
```

Validation errors:

```rust
#[derive(thiserror::Error)]
enum ParseError {
    #[error("missing required key: {0}")]
    MissingKey(&'static str),
    #[error("invalid enum value in `タイプ`")]
    BadType,
    #[error(transparent)]
    Toml(#[from] toml_edit::TomlError),
}
```

### Testing

- `tests/data/*.rs` contain various valid and invalid blocks.
- `cargo nextest run` should cover at least 50 cases.

### Performance Targets

| Metric | Condition | Goal |
|--------|-----------|------|
| Scan speed | 10k files / 1 GB project | < 3s on M2 |
| Memory | Same as above | < 200 MB peak |

## Database Schema (SQLite)

```sql
CREATE TABLE resource (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  path TEXT NOT NULL,
  line_start INTEGER,
  line_end INTEGER,
  name TEXT,
  summary TEXT,
  type TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE tag_master (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  tag TEXT UNIQUE NOT NULL,
  alias_of INTEGER,
  parent_id INTEGER,
  FOREIGN KEY(parent_id) REFERENCES tag_master(id),
  FOREIGN KEY(alias_of) REFERENCES tag_master(id)
);
CREATE TABLE metadata (
  resource_id INTEGER,
  key TEXT,
  value TEXT,
  FOREIGN KEY(resource_id) REFERENCES resource(id)
);
CREATE TABLE resource_tag (
  resource_id INTEGER,
  tag_id INTEGER,
  PRIMARY KEY(resource_id, tag_id),
  FOREIGN KEY(resource_id) REFERENCES resource(id),
  FOREIGN KEY(tag_id) REFERENCES tag_master(id)
);
CREATE INDEX idx_tag ON tag_master(tag);
CREATE INDEX idx_res_tag_tag_id ON resource_tag(tag_id);
```

## CLI Commands

- **import**: `coc r import <path> [--watch] [--delete-orphan]`
- **query**: `coc r query --tag "ai,ml" --type code --json`
- **export**: `coc r export --format csv --out resources.csv`
- **validate**: `coc r validate <file>` (exit code indicates success)
- **tag**: `list | add | alias | rename | merge` (same as v1.2)

## gRPC API (excerpt)

```proto
service CocResource {
  rpc Import (ImportRequest) returns (ImportResponse);
  rpc Query  (QueryRequest)  returns (QueryResponse);
  rpc TagMutate (TagMutateRequest) returns (google.protobuf.Empty);
}
message ImportRequest { string root = 1; bool delete_orphan = 2; }
message ImportResponse { uint32 imported = 1; uint32 updated = 2; }
message QueryRequest  { string tag = 1; string type = 2; }
message QueryResponse { repeated Resource resources = 1; }
```

## Sample Resource Blocks

```
/*
:::RESOURCE-START
名前 = "user_login_flow"
概要 = "メール+パスワードによるログイン処理(バリデーション込み)"
タイプ = "code"

[メタデータ]
author = "akim"
coverage = 0.92

[タグ]
auth = ""
rust = ""
:::RESOURCE-END
*/
```

```
"""
:::RESOURCE-START
名前 = "data_schema_v1"
概要 = "ユーザーテーブルの ER 設計"
タイプ = "doc"

[タグ]
database = ""
erd = ""  # entity-relationship-diagram
:::RESOURCE-END
"""
```

### Adding Comment Styles

Extend `detector::EXTRA_MAP` with the new mapping and add a unit test, e.g.

```rust
{ ".swift", Style { line: "//", block_start: "/*", block_end: "*/" } }
```

END OF SPECIFICATION.

