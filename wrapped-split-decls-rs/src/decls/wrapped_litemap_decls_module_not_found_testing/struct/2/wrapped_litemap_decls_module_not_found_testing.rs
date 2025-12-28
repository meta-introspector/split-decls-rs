use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: testing")] # [llm_context (crate_name = "wrapped_litemap" , module_name = "testing")] pub struct wrapped_litemap_decls_module_not_found_testing ;
}