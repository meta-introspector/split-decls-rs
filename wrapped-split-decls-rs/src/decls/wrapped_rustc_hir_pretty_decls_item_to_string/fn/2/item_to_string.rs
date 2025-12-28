use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: item_to_string");
pub fn item_to_string (ann : & dyn PpAnn , pat : & hir :: Item < '_ >) -> String { to_string (ann , | s | s . print_item (pat)) }
}