use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'graph , G : Predecessors > Predecessors for & 'graph G { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . predecessors (node) } }