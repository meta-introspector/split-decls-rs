use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn read_cargo_toml (path : & Path) -> Result < CargoToml > { let content = fs :: read_to_string (path) . context (format ! ("Failed to read Cargo.toml from {:?}" , path)) ? ; let toml : CargoToml = toml :: from_str (& content) . context (format ! ("Failed to parse Cargo.toml from {:?}" , path)) ? ; Ok (toml) }
}