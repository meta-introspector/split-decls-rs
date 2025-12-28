use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Generate a basic `Cargo.toml`"] pub fn basic_manifest (name : & str , version : & str) -> String { format ! (r#"
        [package]
        name = "{}"
        version = "{}"
        authors = []
        edition = "2015"
    "# , name , version) }