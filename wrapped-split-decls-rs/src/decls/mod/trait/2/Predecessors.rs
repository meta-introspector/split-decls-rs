use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Predecessors : DirectedGraph { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > ; }