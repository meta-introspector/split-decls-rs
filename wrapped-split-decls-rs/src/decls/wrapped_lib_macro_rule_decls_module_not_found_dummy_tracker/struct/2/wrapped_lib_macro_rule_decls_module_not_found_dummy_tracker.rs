use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: dummy_tracker")] # [llm_context (crate_name = "wrapped_lib_macro_rule" , module_name = "dummy_tracker")] pub struct wrapped_lib_macro_rule_decls_module_not_found_dummy_tracker ;
}