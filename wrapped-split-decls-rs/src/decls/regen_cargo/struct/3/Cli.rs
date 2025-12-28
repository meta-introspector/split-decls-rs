use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser)] # [command (name = "regen-cargo")] # [command (about = "Regenerate only Cargo.toml files for existing wrapped workspace")] struct Cli { # [doc = " Verbose output"] # [arg (short , long)] verbose : bool , # [doc = " Output directory (default: output2)"] # [arg (short , long , default_value = "output2")] output_dir : PathBuf , # [doc = " Dry run mode"] # [arg (short , long)] dry_run : bool , # [doc = " Build after regenerating"] # [arg (short , long)] build : bool , }