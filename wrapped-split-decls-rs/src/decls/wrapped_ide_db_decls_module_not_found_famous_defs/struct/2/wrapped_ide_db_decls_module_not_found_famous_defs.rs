use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: famous_defs")] # [llm_context (crate_name = "wrapped_ide_db" , module_name = "famous_defs")] pub struct wrapped_ide_db_decls_module_not_found_famous_defs ;
}