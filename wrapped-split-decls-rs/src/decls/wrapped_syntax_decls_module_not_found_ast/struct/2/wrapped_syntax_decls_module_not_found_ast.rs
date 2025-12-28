use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: ast")] # [llm_context (crate_name = "wrapped_syntax" , module_name = "ast")] pub struct wrapped_syntax_decls_module_not_found_ast ;
}