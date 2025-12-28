use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (clap :: Parser)] struct Cli { # [arg (long , default_value = "output2")] output_dir : PathBuf , # [arg (long)] verbose : bool , # [arg (long)] build : bool , # [arg (long)] dry_run : bool , # [arg (long)] analyze_only : bool , }
}