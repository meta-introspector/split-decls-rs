use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "atomic_swap" , vis = "pub" , hash = "fe650a05")] pub fn atomic_swap (input : TokenStream) -> TokenStream { mev_protection :: atomic_swap_impl (input) }
}