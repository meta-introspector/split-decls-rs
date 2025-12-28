use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser , Debug)] # [clap (author , version , about = "Wrap a single crate with minimal configuration")] struct Args { # [doc = " Path to the crate directory (containing Cargo.toml)"] crate_path : PathBuf , # [doc = " Output directory (optional, defaults to ./output2)"] # [clap (short , long)] output : Option < PathBuf > , # [doc = " Verbose output"] # [clap (short , long)] verbose : bool , }