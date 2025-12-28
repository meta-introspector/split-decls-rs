use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: ca");
# [proc_macro] pub fn ca (input : TokenStream) -> TokenStream { solana_lift :: ca_macro_impl (input) }
}