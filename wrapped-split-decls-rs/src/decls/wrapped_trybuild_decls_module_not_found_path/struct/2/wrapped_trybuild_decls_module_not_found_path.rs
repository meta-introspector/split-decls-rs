use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: path")] # [llm_context (crate_name = "wrapped_trybuild" , module_name = "path")] pub struct wrapped_trybuild_decls_module_not_found_path ;
}