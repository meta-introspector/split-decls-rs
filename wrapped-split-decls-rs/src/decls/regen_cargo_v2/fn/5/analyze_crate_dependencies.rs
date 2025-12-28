use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn analyze_crate_dependencies (cargo_toml_path : & PathBuf , crate_name : & str , analysis : & mut DependencyAnalysis , global_deps : & mut HashMap < String , Vec < (String , String) > >) -> Result < () > { let content = std :: fs :: read_to_string (cargo_toml_path) ? ; let cargo_toml : toml :: Value = toml :: from_str (& content) ? ; let sections = ["dependencies" , "dev-dependencies" , "build-dependencies"] ; for section in & sections { if let Some (deps) = cargo_toml . get (section) . and_then (| v | v . as_table ()) { for (dep_name , dep_value) in deps { let dep_spec = analyze_dependency_spec (dep_name , dep_value , analysis) ; global_deps . entry (dep_name . clone ()) . or_insert_with (Vec :: new) . push ((format ! ("{}[{}]" , crate_name , section) , dep_spec)) ; } } } Ok (()) }
}