use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn load_config () -> Result < SplitDeclsConfig > { let config_path = "split-decls-rs.toml" ; let config_content = std :: fs :: read_to_string (config_path) ? ; let config : SplitDeclsConfig = toml :: from_str (& config_content) ? ; Ok (config) }
}