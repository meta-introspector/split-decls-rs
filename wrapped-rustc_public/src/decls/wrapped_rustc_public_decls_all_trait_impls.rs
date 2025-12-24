use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn all_trait_impls() -> ImplTraitDecls {
    with(|cx| cx.all_trait_impls())
}
