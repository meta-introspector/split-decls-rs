use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "archive_event" , vis = "pub" , hash = "db2cb06b")] pub fn archive_event (input : TokenStream) -> TokenStream { event_memory :: archive_event_impl (input) }
}