use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn find_rust_files (dir : & Path) -> Vec < std :: path :: PathBuf > { let mut rust_files = Vec :: new () ; if let Ok (entries) = fs :: read_dir (dir) { for entry in entries . flatten () { let path = entry . path () ; if path . is_dir () && ! path . file_name () . unwrap_or_default () . to_string_lossy () . starts_with ('.') { rust_files . extend (find_rust_files (& path)) ; } else if path . extension () . map_or (false , | ext | ext == "rs") { rust_files . push (path) ; } } } rust_files }