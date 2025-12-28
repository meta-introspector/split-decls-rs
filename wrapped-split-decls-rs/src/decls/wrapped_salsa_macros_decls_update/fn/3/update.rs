use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: update");
# [proc_macro_derive (Update , attributes (update))] pub fn update (input : TokenStream) -> TokenStream { let item = parse_macro_input ! (input as syn :: DeriveInput) ; match update :: update_derive (item) { Ok (tokens) => tokens . into () , Err (error) => error . into_compile_error () . into () , } }
}