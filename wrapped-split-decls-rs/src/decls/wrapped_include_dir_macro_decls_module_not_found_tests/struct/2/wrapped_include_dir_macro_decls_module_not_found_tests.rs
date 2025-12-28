use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: tests")] # [llm_context (crate_name = "wrapped_include_dir_macro" , module_name = "tests")] pub struct wrapped_include_dir_macro_decls_module_not_found_tests ;
}