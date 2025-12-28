use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: tests")] # [llm_context (crate_name = "wrapped_lsp_server" , module_name = "tests")] pub struct wrapped_lsp_server_decls_module_not_found_tests ;
}