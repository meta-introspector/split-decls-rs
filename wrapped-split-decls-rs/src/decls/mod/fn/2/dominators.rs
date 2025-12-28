use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn dominators < G : ControlFlowGraph > (g : & G) -> Dominators < G :: Node > { if is_small_path_graph (g) { Dominators { kind : Kind :: Path } } else { Dominators { kind : Kind :: General (dominators_impl (g)) } } }
}