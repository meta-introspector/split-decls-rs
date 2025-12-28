use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: foreign")] # [llm_context (crate_name = "wrapped_arbitrary" , module_name = "foreign")] pub struct wrapped_arbitrary_decls_module_not_found_foreign ;
}