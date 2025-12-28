use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Write a Cargo.toml file"] pub fn write_manifest (path : & Path , manifest : & Value) -> Result < () > { let content = toml :: to_string_pretty (manifest) ? ; std :: fs :: write (path , content) ? ; Ok (()) }