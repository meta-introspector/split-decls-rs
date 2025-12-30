use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: ceprintln");
# [doc = " The same as `eprintln!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn ceprintln (input : TokenStream) -> TokenStream { get_macro ("eprintln" , input , false) }
}