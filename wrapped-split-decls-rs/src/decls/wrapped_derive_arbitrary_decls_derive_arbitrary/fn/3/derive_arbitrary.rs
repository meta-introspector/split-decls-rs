use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: derive_arbitrary");
# [proc_macro_derive (Arbitrary , attributes (arbitrary))] pub fn derive_arbitrary (tokens : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = syn :: parse_macro_input ! (tokens as syn :: DeriveInput) ; expand_derive_arbitrary (input) . unwrap_or_else (syn :: Error :: into_compile_error) . into () }
}