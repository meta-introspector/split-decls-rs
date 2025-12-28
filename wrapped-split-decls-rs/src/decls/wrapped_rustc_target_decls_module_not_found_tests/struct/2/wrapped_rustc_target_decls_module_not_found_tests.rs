use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: tests")] # [llm_context (crate_name = "wrapped_rustc_target" , module_name = "tests")] pub struct wrapped_rustc_target_decls_module_not_found_tests ;
}