use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: mkbuildrs");
# [proc_macro] pub fn mkbuildrs (input : TokenStream) -> TokenStream { mkbuildrs :: mkbuildrs_impl (input) }
}