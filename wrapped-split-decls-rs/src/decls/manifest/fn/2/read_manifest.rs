use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Read and parse a Cargo.toml file"] pub fn read_manifest (path : & Path) -> Result < Value > { let content = std :: fs :: read_to_string (path) ? ; let manifest : Value = toml :: from_str (& content) ? ; Ok (manifest) }
}