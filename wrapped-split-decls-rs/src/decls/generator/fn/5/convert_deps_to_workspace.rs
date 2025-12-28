use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: convert_deps_to_workspace");
# [doc = " Convert all dependencies to use workspace = true"] fn convert_deps_to_workspace (manifest : & mut Value) -> Result < () > { let special_crates = ["alloc" , "core" , "std" , "proc_macro"] ; for section in ["dependencies" , "dev-dependencies" , "build-dependencies"] { if let Some (deps) = manifest . get_mut (section) . and_then (| v | v . as_table_mut ()) { for (name , value) in deps . iter_mut () { if special_crates . contains (& name . as_str ()) { continue ; } if let Some (table) = value . as_table_mut () { table . remove ("version") ; table . remove ("path") ; table . insert ("workspace" . to_string () , Value :: Boolean (true)) ; } } } } Ok (()) }
}