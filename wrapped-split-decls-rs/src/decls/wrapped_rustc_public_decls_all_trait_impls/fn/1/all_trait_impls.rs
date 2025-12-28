use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn all_trait_impls () -> ImplTraitDecls { with (| cx | cx . all_trait_impls ()) }
}