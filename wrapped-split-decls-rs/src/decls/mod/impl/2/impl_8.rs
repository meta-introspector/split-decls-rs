use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < N : Idx , const BR : bool > NumEdges for VecGraph < N , BR > { fn num_edges (& self) -> usize { match BR { false => self . edge_targets . len () , true => self . edge_targets . len () / 2 , } } }