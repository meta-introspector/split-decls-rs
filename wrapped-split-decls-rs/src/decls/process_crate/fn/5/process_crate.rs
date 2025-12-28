use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Process a single crate by applying the split-decls transformation."] pub fn process_crate (crate_path : & Path , global_config : & SplitDeclsConfig , dry_run : bool) -> Result < () > { let paths = setup_crate_paths (crate_path) ? ; if dry_run { println ! ("DRY RUN: Would process crate at {}" , crate_path . display ()) ; return Ok (()) ; } println ! ("Processing crate: {}" , paths . crate_name) ; let lib_rs_path = paths . source_files . iter () . find (| p | p . file_name () . and_then (| n | n . to_str ()) == Some ("lib.rs")) ; if lib_rs_path . is_none () || ! lib_rs_path . unwrap () . exists () { println ! ("No lib.rs found in source files, skipping") ; return Ok (()) ; } eager_split_crate (& paths , global_config) ? ; println ! ("Successfully processed crate: {}" , paths . crate_name) ; Ok (()) }
}