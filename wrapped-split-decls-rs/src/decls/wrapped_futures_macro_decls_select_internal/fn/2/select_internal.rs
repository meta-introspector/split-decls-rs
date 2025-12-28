use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " The `select!` macro."] # [proc_macro] pub fn select_internal (input : TokenStream) -> TokenStream { crate :: select :: select (input) }
}