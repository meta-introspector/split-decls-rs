use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: tester")] # [llm_context (crate_name = "wrapped_quickcheck" , module_name = "tester")] pub struct wrapped_quickcheck_decls_module_not_found_tester ;
}