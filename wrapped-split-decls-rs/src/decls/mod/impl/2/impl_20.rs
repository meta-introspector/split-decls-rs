use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < N : Idx , S : Idx + Ord > NumEdges for Sccs < N , S > { fn num_edges (& self) -> usize { self . scc_data . all_successors . len () } }
}