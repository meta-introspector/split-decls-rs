use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Scan all Rust source files in a crate and extract functions"] pub fn scan_all_rust_files (crate_path : & Path) -> Result < Vec < (PathBuf , Vec < Item >) > > { let mut all_files = Vec :: new () ; let src_dir = crate_path . join ("src") ; if src_dir . exists () { scan_rust_files_recursive (& src_dir , & mut all_files) ? ; } Ok (all_files) }
}