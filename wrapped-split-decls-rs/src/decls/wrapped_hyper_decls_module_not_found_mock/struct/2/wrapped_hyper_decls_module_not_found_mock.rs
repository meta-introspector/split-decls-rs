use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: mock")] # [llm_context (crate_name = "wrapped_hyper" , module_name = "mock")] pub struct wrapped_hyper_decls_module_not_found_mock ;
}