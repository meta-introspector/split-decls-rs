use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Needs { fn maybe_mut_place (m : hir :: Mutability) -> Self { match m { hir :: Mutability :: Mut => Needs :: MutPlace , hir :: Mutability :: Not => Needs :: None , } } }
}