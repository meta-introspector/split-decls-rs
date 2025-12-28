use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] pub fn quote_error (input : TokenStream) -> TokenStream { input }
}