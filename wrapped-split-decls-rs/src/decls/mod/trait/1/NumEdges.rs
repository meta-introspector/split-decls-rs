use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait NumEdges : DirectedGraph { fn num_edges (& self) -> usize ; }
}