use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait NumEdges : DirectedGraph { fn num_edges (& self) -> usize ; }