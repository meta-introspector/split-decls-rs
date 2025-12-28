use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser , Debug)] # [command (author , version , about , long_about = None)] struct Args { # [doc = " Path to the parent workspace's Cargo.toml"] # [arg (short , long)] parent_workspace_root : PathBuf , # [doc = " Path to the child crate that should be merged into the parent workspace"] # [arg (short , long)] child_crate_root : PathBuf , }