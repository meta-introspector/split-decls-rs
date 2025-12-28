use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct CratePaths { pub crate_path : PathBuf , pub crate_name : String , pub lib_rs_path : PathBuf , pub build_rs_path : PathBuf , pub cargo_toml_path : PathBuf , pub decls_output_dir : PathBuf , pub target_config_path : PathBuf , pub output_crate_path : PathBuf , }