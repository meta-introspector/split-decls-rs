use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro_derive (Decode , attributes (bincode))] pub fn derive_decode (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_decode_inner (input) . unwrap_or_else (| e | e . into_token_stream ()) }
}