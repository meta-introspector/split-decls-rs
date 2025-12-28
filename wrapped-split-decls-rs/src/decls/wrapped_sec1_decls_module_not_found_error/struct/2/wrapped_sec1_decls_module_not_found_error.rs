use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: error")] # [llm_context (crate_name = "wrapped_sec1" , module_name = "error")] pub struct wrapped_sec1_decls_module_not_found_error ;
}