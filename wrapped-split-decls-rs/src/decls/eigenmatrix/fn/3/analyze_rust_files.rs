use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn analyze_rust_files (dir : & Path , metrics : & mut CrateMetrics) -> Result < () > { for entry in std :: fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { let content = std :: fs :: read_to_string (& path) ? ; analyze_rust_content (& content , metrics) ; } else if path . is_dir () { analyze_rust_files (& path , metrics) ? ; } } Ok (()) }
}