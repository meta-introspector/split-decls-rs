use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: db")] # [llm_context (crate_name = "wrapped_hir_def" , module_name = "db")] pub struct wrapped_hir_def_decls_module_not_found_db ;
}