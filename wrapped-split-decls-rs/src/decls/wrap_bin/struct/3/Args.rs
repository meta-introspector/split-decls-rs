use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Parser)] # [command (name = "wrap-bin")] # [command (about = "Generate wrapped binary main.rs and Cargo.toml in output2")] struct Args { # [doc = " Name of the binary to wrap (e.g., \"split-decls-rs\")"] binary_name : String , # [doc = " Output directory (default: output2)"] # [arg (short , long , default_value = "output2")] output_dir : PathBuf , # [doc = " Verbose output"] # [arg (short , long)] verbose : bool , }
}