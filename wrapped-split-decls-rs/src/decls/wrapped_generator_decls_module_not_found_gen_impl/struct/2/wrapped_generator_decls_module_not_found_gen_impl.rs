use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [llm_error_message (message = "Module file not found for: gen_impl")] # [llm_context (crate_name = "wrapped_generator" , module_name = "gen_impl")] pub struct wrapped_generator_decls_module_not_found_gen_impl ;
}