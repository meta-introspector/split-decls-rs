use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: path")] # [llm_context (crate_name = "wrapped_zip" , module_name = "path")] pub struct wrapped_zip_decls_module_not_found_path ;
}