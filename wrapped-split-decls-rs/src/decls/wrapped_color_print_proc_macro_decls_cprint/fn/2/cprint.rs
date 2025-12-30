use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: cprint");
# [doc = " The same as `print!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cprint (input : TokenStream) -> TokenStream { get_macro ("print" , input , false) }
}