use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < N : Idx , const BR : bool > DirectedGraph for VecGraph < N , BR > { type Node = N ; fn num_nodes (& self) -> usize { match BR { false => self . node_starts . len () - 1 , true => (self . node_starts . len () - 1) / 2 , } } }