use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "mkbuildrs_with_macros" , vis = "pub" , hash = "4bdbc81f")] pub fn mkbuildrs_with_macros (input : TokenStream) -> TokenStream { macro_generator :: mkbuildrs_with_macros_impl (input) }
}