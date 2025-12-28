use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: ted")] # [llm_context (crate_name = "wrapped_syntax" , module_name = "ted")] pub struct wrapped_syntax_decls_module_not_found_ted ;
}