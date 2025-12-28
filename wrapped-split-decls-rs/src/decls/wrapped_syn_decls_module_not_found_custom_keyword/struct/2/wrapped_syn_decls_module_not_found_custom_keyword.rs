use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: custom_keyword")] # [llm_context (crate_name = "wrapped_syn" , module_name = "custom_keyword")] pub struct wrapped_syn_decls_module_not_found_custom_keyword ;
}