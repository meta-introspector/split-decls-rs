use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "euler" , vis = "pub" , hash = "e63b3b8a")] pub fn euler (input : TokenStream) -> TokenStream { macros :: euler :: euler_impl (input) }
}