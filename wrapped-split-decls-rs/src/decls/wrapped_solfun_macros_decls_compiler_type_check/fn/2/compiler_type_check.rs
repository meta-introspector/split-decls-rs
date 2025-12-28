use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "compiler_type_check" , vis = "pub" , hash = "a74e17ed")] pub fn compiler_type_check (input : TokenStream) -> TokenStream { macros :: compiler_type_check :: compiler_type_check_impl (input) }
}