use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn traced_setup_crate_paths (crate_path : & Path) -> Result < CratePaths > { trace_call ! ("setup_crate_paths" , setup_crate_paths (crate_path)) }