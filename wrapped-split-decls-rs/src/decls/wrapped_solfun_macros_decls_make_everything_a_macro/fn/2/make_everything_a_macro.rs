use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "make_everything_a_macro" , vis = "pub" , hash = "69a030ad")] pub fn make_everything_a_macro (input : TokenStream) -> TokenStream { macros :: make_everything_a_macro :: make_everything_a_macro_impl (input) }
}