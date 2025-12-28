use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: defs")] # [llm_context (crate_name = "wrapped_ide_db" , module_name = "defs")] pub struct wrapped_ide_db_decls_module_not_found_defs ;
}