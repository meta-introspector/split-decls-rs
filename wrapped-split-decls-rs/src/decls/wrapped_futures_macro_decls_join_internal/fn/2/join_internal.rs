use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " The `join!` macro."] # [proc_macro] pub fn join_internal (input : TokenStream) -> TokenStream { crate :: join :: join (input) }
}