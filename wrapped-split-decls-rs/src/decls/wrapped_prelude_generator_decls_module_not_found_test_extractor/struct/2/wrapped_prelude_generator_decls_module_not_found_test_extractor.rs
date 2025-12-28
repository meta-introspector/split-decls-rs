use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: test_extractor")] # [llm_context (crate_name = "wrapped_prelude_generator" , module_name = "test_extractor")] pub struct wrapped_prelude_generator_decls_module_not_found_test_extractor ;
}