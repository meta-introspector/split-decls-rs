use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (super) struct TestGraph { num_nodes : usize , start_node : usize , successors : FxHashMap < usize , Vec < usize > > , predecessors : FxHashMap < usize , Vec < usize > > , }