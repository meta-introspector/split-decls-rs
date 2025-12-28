use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: target")] # [llm_context (crate_name = "wrapped_base_db" , module_name = "target")] pub struct wrapped_base_db_decls_module_not_found_target ;
}