use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "lp" , vis = "pub" , hash = "681b5eb9")] pub fn lp (input : TokenStream) -> TokenStream { solana_lift :: lp_macro_impl (input) }
}