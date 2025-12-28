use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < G : Predecessors > Successors for ReversedGraph < G > { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . inner . predecessors (node) } }
}