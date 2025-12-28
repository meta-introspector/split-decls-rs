use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < N : Idx , S : Idx + Ord > DirectedGraph for Sccs < N , S > { type Node = S ; fn num_nodes (& self) -> usize { self . num_sccs () } }
}