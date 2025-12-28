use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: fixture")] # [llm_context (crate_name = "wrapped_test_utils" , module_name = "fixture")] pub struct wrapped_test_utils_decls_module_not_found_fixture ;
}