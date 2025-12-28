use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn traced_generate_wrapped_crate (wrapped_workspace_root : & Path , original_crate_name : & str , original_crate_path : & Path , global_config : & SplitDeclsConfig , patch_config : & PatchConfig , dry_run : bool , cargo_only : bool ,) -> Result < Vec < ModuleNotFoundReport > > { trace_call ! ("generate_wrapped_crate" , generate_wrapped_crate (wrapped_workspace_root , original_crate_name , original_crate_path , global_config , patch_config , dry_run , cargo_only)) }