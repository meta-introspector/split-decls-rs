use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: hir")] # [llm_context (crate_name = "wrapped_hir_def" , module_name = "hir")] pub struct wrapped_hir_def_decls_module_not_found_hir ;
}