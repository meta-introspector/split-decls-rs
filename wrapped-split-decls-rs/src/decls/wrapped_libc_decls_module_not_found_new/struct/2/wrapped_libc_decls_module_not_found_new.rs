use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: new")] # [llm_context (crate_name = "wrapped_libc" , module_name = "new")] pub struct wrapped_libc_decls_module_not_found_new ;
}