use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "telegram" , vis = "pub" , hash = "378965a5")] pub fn telegram (input : TokenStream) -> TokenStream { macros :: telegram :: telegram_impl (input) }
}