use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl TraitId { # [inline] pub fn trait_items (self , db : & dyn DefDatabase) -> & TraitItems { TraitItems :: query (db , self) } }
}