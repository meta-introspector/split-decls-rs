use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: lang_item")] # [llm_context (crate_name = "wrapped_hir_def" , module_name = "lang_item")] pub struct wrapped_hir_def_decls_module_not_found_lang_item ;
}