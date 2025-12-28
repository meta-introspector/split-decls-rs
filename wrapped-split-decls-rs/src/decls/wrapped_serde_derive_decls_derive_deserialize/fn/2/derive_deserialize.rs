use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro_derive (Deserialize , attributes (serde))] pub fn derive_deserialize (input : TokenStream) -> TokenStream { let mut input = parse_macro_input ! (input as DeriveInput) ; de :: expand_derive_deserialize (& mut input) . unwrap_or_else (syn :: Error :: into_compile_error) . into () }
}