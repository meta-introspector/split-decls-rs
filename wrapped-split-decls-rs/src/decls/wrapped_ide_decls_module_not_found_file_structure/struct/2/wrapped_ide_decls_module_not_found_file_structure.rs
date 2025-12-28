use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: file_structure")] # [llm_context (crate_name = "wrapped_ide" , module_name = "file_structure")] pub struct wrapped_ide_decls_module_not_found_file_structure ;
}