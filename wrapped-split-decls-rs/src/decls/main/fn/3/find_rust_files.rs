use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn find_rust_files (crate_path : & Path) -> Vec < PathBuf > { WalkDir :: new (crate_path) . into_iter () . filter_map (| e | e . ok ()) . filter (| e | { let path = e . path () ; if path . to_string_lossy () . contains ("output2") || path . to_string_lossy () . contains ("enhanced_output") { return false ; } path . extension () . map_or (false , | ext | ext == "rs") }) . map (| e | e . path () . to_path_buf ()) . collect () }
}