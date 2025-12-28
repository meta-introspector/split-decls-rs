use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: all_trait_impls");
pub fn all_trait_impls () -> ImplTraitDecls { with (| cx | cx . all_trait_impls ()) }
}