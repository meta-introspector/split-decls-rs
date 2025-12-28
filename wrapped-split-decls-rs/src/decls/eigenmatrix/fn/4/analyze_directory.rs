use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn analyze_directory (path : & Path , crates : & mut Vec < CrateMetrics >) -> Result < () > { if ! path . exists () { return Err (anyhow :: anyhow ! ("Path does not exist: {:?}" , path)) ; } for entry in std :: fs :: read_dir (path) ? { let entry = entry ? ; let entry_path = entry . path () ; if entry_path . is_dir () { let cargo_toml = entry_path . join ("Cargo.toml") ; if cargo_toml . exists () { let crate_name = entry_path . file_name () . unwrap_or_default () . to_string_lossy () . to_string () ; let metrics = analyze_crate (& entry_path , crate_name) ? ; crates . push (metrics) ; } analyze_directory (& entry_path , crates) ? ; } } Ok (()) }
}