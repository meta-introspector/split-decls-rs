use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type TraitLoc = ItemLoc < ast :: Trait > ;
}