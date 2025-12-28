use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: def_use")] # [llm_context (crate_name = "wrapped_rustc_borrowck" , module_name = "def_use")] pub struct wrapped_rustc_borrowck_decls_module_not_found_def_use ;
}