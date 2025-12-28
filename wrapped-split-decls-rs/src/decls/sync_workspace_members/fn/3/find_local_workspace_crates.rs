use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn find_local_workspace_crates (dir : & Path) -> Result < Vec < String > > { let mut crates = Vec :: new () ; for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_dir () { let cargo_toml = path . join ("Cargo.toml") ; if cargo_toml . exists () { if let Some (name) = path . file_name () { crates . push (name . to_string_lossy () . to_string ()) ; } } } } crates . sort () ; Ok (crates) }