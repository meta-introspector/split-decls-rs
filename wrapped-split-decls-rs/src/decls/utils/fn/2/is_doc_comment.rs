use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: is_doc_comment");
pub (super) fn is_doc_comment (attr : & Attribute) -> bool { attr . path () . segments . last () . unwrap () . ident == "doc" }
}