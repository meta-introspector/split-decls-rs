use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: tests")] # [llm_context (crate_name = "wrapped_syntax_bridge" , module_name = "tests")] pub struct wrapped_syntax_bridge_decls_module_not_found_tests ;
}