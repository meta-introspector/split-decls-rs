use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: syntax_node")] # [llm_context (crate_name = "wrapped_syntax" , module_name = "syntax_node")] pub struct wrapped_syntax_decls_module_not_found_syntax_node ;
}