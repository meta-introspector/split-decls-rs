use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct CliArgs { pub dry_run : bool , pub verbose : bool , pub wrapped_workspace_output_dir : PathBuf , pub patch_config_path_str : String , pub generate_wrapped_workspace_mode : bool , }
}