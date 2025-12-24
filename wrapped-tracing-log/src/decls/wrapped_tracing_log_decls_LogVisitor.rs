use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct LogVisitor<'a> {
    target: Option<&'a str>,
    module_path: Option<&'a str>,
    file: Option<&'a str>,
    line: Option<u64>,
    fields: &'static Fields,
}
