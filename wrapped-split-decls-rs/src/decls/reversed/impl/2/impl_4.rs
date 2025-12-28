use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < G : Predecessors > Successors for ReversedGraph < G > { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . inner . predecessors (node) } }