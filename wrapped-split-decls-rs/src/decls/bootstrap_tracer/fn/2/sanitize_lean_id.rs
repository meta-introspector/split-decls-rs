use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn sanitize_lean_id (id : & str) -> String { id . chars () . map (| c | if c . is_alphanumeric () || c == '_' { c } else { '_' }) . collect :: < String > () . trim_start_matches (| c : char | c . is_numeric ()) . to_string () }
}