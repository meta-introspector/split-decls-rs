use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < N : Idx + Ord > Predecessors for VecGraph < N , true > { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . predecessors (node) . iter () . cloned () } }