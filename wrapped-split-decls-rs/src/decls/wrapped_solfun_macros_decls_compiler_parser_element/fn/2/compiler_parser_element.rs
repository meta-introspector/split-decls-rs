use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "compiler_parser_element" , vis = "pub" , hash = "df0f80a4")] pub fn compiler_parser_element (input : TokenStream) -> TokenStream { macros :: compiler_parser_element :: compiler_parser_element_impl (input) }
}