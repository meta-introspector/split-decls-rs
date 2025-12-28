use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , serde :: Serialize , serde :: Deserialize)] pub struct CargoToml { pub package : super :: package :: Package , pub lib : Option < toml :: Table > , # [serde (default)] pub dependencies : toml :: Table , # [serde (rename = "dev-dependencies")] # [serde (default)] pub dev_dependencies : toml :: Table , # [serde (rename = "build-dependencies")] # [serde (default)] pub build_dependencies : toml :: Table , # [serde (flatten)] # [serde (default)] pub other : toml :: Table , # [serde (default)] pub patch : toml :: Table , }
}