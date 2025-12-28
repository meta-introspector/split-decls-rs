use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser)] # [command (name = "eigenmatrix")] # [command (about = "Pure Rust eigenmatrix analysis of any codebase")] struct Args { # [doc = " Path to analyze"] path : PathBuf , }