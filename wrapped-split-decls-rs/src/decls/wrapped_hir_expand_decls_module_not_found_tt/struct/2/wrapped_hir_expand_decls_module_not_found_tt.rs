use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: tt")] # [llm_context (crate_name = "wrapped_hir_expand" , module_name = "tt")] pub struct wrapped_hir_expand_decls_module_not_found_tt ;
}