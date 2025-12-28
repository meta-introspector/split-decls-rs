use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: tests")] # [llm_context (crate_name = "wrapped_uluru" , module_name = "tests")] pub struct wrapped_uluru_decls_module_not_found_tests ;
}