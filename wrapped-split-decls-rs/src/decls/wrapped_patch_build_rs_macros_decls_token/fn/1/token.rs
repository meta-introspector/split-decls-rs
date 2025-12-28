use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] pub fn token (input : TokenStream) -> TokenStream { solana_lift :: token_macro_impl (input) }
}