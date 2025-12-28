use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn process_crates_in_path (root_path : & Path , current_crate_name : & str , _global_config : & SplitDeclsConfig , _is_rustc_source : bool , dry_run : bool ,) -> Result < () > { if dry_run { println ! ("DRY RUN: Would process crates in {}" , root_path . display ()) ; return Ok (()) ; } println ! ("Processing crates in path: {}" , root_path . display ()) ; let cargo_toml_path = root_path . join ("Cargo.toml") ; if cargo_toml_path . exists () { let crate_name = root_path . file_name () . and_then (| n | n . to_str ()) . unwrap_or ("unknown") ; if crate_name != current_crate_name { println ! ("Would process crate: {}" , crate_name) ; } } Ok (()) }
}