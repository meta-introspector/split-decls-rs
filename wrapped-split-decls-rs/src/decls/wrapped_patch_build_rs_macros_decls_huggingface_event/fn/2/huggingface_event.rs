use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "huggingface_event" , vis = "pub" , hash = "18e2ab0a")] pub fn huggingface_event (input : TokenStream) -> TokenStream { event_memory :: huggingface_event_impl (input) }
}