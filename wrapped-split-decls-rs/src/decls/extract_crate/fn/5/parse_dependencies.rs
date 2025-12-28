use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: parse_dependencies");
# [doc = " Parse dependencies from Cargo.toml"] fn parse_dependencies (cargo_toml : & Path) -> Result < (HashSet < String > , HashSet < String >) > { let content = fs :: read_to_string (cargo_toml) ? ; let toml : Value = toml :: from_str (& content) ? ; let mut deps = HashSet :: new () ; let mut dev_deps = HashSet :: new () ; if let Some (dependencies) = toml . get ("dependencies") . and_then (| v | v . as_table ()) { for (name , _) in dependencies { deps . insert (name . clone ()) ; } } if let Some (dev_dependencies) = toml . get ("dev-dependencies") . and_then (| v | v . as_table ()) { for (name , _) in dev_dependencies { dev_deps . insert (name . clone ()) ; } } Ok ((deps , dev_deps)) }
}