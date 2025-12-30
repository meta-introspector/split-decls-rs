use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: cwriteln");
# [doc = " The same as `writeln!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cwriteln (input : TokenStream) -> TokenStream { get_macro ("writeln" , input , true) }
}