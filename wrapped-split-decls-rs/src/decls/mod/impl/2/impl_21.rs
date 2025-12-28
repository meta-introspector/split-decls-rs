use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < N : Idx , S : Idx + Ord > Successors for Sccs < N , S > { fn successors (& self , node : S) -> impl Iterator < Item = Self :: Node > { self . successors (node) . iter () . cloned () } }
}