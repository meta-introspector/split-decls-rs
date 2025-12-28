use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn format_rust_file (content : & str , path : & Path) -> Result < String > { if std :: env :: var ("SPLIT_DECLS_DEBUG") . is_ok () { eprintln ! ("DEBUG: Skipping formatting for {}" , path . display ()) ; } Ok (content . to_string ()) }
}