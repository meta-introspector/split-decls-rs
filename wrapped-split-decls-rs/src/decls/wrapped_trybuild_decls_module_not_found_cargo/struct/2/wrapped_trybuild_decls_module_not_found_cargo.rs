use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: cargo")] # [llm_context (crate_name = "wrapped_trybuild" , module_name = "cargo")] pub struct wrapped_trybuild_decls_module_not_found_cargo ;
}