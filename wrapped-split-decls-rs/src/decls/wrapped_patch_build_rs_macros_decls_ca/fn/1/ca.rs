use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] pub fn ca (input : TokenStream) -> TokenStream { solana_lift :: ca_macro_impl (input) }
}