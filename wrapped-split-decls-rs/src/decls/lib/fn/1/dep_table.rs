use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] pub fn dep_table (input : TokenStream) -> TokenStream { macros :: dep_table_impl (input) }
}