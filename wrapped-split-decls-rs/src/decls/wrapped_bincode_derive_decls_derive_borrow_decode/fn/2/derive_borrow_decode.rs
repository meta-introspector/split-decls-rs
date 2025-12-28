use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro_derive (BorrowDecode , attributes (bincode))] pub fn derive_borrow_decode (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_borrow_decode_inner (input) . unwrap_or_else (| e | e . into_token_stream ()) }
}