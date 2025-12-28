use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Parser , Debug)] # [clap (author , version , about , long_about = None)] struct Args { # [doc = " Path to the split-decls-rs.toml configuration file to update"] # [clap (long)] split_decls_config_path : PathBuf , }
}