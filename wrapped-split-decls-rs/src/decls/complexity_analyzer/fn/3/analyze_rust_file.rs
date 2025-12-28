use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn analyze_rust_file (file_path : & Path) -> Result < Vec < ComplexityReport > > { let content = fs :: read_to_string (file_path) ? ; let syntax_tree : File = syn :: parse_file (& content) ? ; let mut reports = Vec :: new () ; for item in syntax_tree . items { if let Some (report) = analyze_complexity (& item , file_path . to_string_lossy () . as_ref ()) { reports . push (report) ; } } Ok (reports) }
}