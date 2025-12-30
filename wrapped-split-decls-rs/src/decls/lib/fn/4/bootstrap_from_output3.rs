use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: bootstrap_from_output3");
pub fn bootstrap_from_output3 (output3_dir : & Path , output4_dir : & Path) -> Result < () , Box < dyn std :: error :: Error > > { println ! ("🚀 Bootstrap4: Using WRAPPED functions from output3 → output4") ; wrapped_fs_create_dir_all ! (output4_dir) ? ; for entry in wrapped_fs_read_dir ! (output3_dir) ? { let entry = entry ? ; if entry . file_type () ? . is_dir () { let crate_path = entry . path () ; if let Err (e) = process_crate (& crate_path , output4_dir) { println ! ("⚠️  Skipped {}: {}" , crate_path . display () , e) ; } } } println ! ("🎉 Bootstrap4 complete using ALL WRAPPED functions!") ; Ok (()) }
}