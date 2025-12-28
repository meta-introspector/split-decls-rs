use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn reverse_map_prefix (mapping : & FilePathMapping , p : & str) -> Option < String > { mapping . reverse_map_prefix_heuristically (& path (p)) . map (| q | q . to_string_lossy () . to_string ()) }
}