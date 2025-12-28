use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: derive_serialize");
# [proc_macro_derive (Serialize , attributes (serde))] pub fn derive_serialize (input : TokenStream) -> TokenStream { let mut input = parse_macro_input ! (input as DeriveInput) ; ser :: expand_derive_serialize (& mut input) . unwrap_or_else (syn :: Error :: into_compile_error) . into () }
}