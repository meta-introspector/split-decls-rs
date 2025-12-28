use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: ule_derive");
# [doc = " Full docs for this proc macro can be found on the [`zerovec`](https://docs.rs/zerovec) crate."] # [proc_macro_derive (ULE)] pub fn ule_derive (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; TokenStream :: from (ule :: derive_impl (& input)) }
}