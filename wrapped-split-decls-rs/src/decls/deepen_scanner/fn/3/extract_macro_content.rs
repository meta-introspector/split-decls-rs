use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: extract_macro_content");
fn extract_macro_content (line : & str) -> Option < String > { if let Some (content_start) = line . rfind ('"') { if let Some (content_end) = line [.. content_start] . rfind ('"') { return Some (line [content_end + 1 .. content_start] . to_string ()) ; } } None }
}