use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser)] # [command (name = "gen-workspace")] # [command (about = "Generate workspace Cargo.toml using split-decls-rs.toml configuration")] struct Cli { # [doc = " Output directory (default: output2)"] # [arg (short , long , default_value = "output2")] output_dir : PathBuf , }