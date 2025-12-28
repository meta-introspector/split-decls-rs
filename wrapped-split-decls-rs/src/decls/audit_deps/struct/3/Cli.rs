use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser)] # [command (name = "audit-deps")] # [command (about = "Audit all dependencies and ensure they resolve to local submodules")] struct Cli { # [doc = " Verbose output"] # [arg (short , long)] verbose : bool , # [doc = " Output directory (default: output2)"] # [arg (short , long , default_value = "output2")] output_dir : PathBuf , # [doc = " Fix dependencies automatically"] # [arg (short , long)] fix : bool , }