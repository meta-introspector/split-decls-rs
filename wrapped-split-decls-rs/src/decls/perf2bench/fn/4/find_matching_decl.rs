use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn find_matching_decl (function_name : & str , output2_path : & Path) -> Option < String > { let crate_name = function_name . split ("::") . next () ? ; let wrapped_dir = output2_path . join (format ! ("wrapped-{}" , crate_name)) ; if ! wrapped_dir . exists () { return None ; } let decls_dir = wrapped_dir . join ("src/decls") ; if let Ok (entries) = fs :: read_dir (& decls_dir) { for entry in entries . flatten () { let file_name_string = entry . file_name () . to_string_lossy () . to_string () ; if file_name_string . contains (& function_name . replace ("::" , "_")) { return Some (entry . path () . to_string_lossy () . to_string ()) ; } } } None }
}