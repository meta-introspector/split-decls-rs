use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: goto_declaration")] # [llm_context (crate_name = "wrapped_ide" , module_name = "goto_declaration")] pub struct wrapped_ide_decls_module_not_found_goto_declaration ;
}