use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'graph , G : Successors > Successors for & 'graph G { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . successors (node) } }
}