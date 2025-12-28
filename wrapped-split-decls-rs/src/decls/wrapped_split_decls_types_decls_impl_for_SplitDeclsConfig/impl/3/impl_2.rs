use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl SplitDeclsConfig { pub fn load_from_file (path : & std :: path :: Path) -> anyhow :: Result < Self > { if ! path . exists () { println ! ("No split-decls-rs.toml found at {}, using default configuration." , path . display ()) ; return Ok (Self :: default ()) ; } let content = std :: fs :: read_to_string (path) ? ; let config : Self = toml :: from_str (& content) ? ; Ok (config) } }
}