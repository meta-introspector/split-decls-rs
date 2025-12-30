use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: process_decl2_attribute_logic");
pub fn process_decl2_attribute_logic (attr : TokenStream , item : TokenStream) -> TokenStream { let args = parse_decl_args ! (attr) ; dispatch_wrap_logic ! (item , args) }
}