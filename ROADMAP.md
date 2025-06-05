# Development Roadmap

This roadmap outlines the module responsibilities and completion criteria derived from section 10 of "coc-resource 仕様書 v1.3".

| No | Module                | Assigned Agent | Completion Criteria |
|----|-----------------------|---------------|--------------------|
| 1  | detector/extractor    | Rust Agent A  | All tests pass (`Green`) |
| 2  | TOML parser & validator | Rust Agent B | Error variants fully implemented |
| 3  | DB layer (`db::store`) | Rust Agent C | CRUD + migration working |
| 4  | CLI integration (`clap`) | Rust Agent D | `coc r help` runs correctly |
| 5  | Tag operations        | Rust Agent E  | list/add/alias tests succeed |
| 6  | gRPC server           | Rust Agent F  | `grpcurl` invocation works |
| 7  | docs/examples         | Agent Docu    | `README` and sample blocks complete |

Each stage can proceed independently but should pass unit tests before moving to the next.

