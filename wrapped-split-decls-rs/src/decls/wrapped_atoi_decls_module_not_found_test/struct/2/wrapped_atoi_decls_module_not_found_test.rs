use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: test")] # [llm_context (crate_name = "wrapped_atoi" , module_name = "test")] pub struct wrapped_atoi_decls_module_not_found_test ;
}