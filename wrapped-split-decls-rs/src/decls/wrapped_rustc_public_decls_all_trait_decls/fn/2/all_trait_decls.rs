use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: all_trait_decls");
pub fn all_trait_decls () -> TraitDecls { with (| cx | cx . all_trait_decls ()) }
}