use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < G : Successors > Predecessors for ReversedGraph < G > { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . inner . successors (node) } }