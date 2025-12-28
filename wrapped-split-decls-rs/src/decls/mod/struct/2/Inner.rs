use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Tracks the list of dominators for each node."] # [derive (Clone , Debug)] struct Inner < N : Idx > { immediate_dominators : IndexVec < N , Option < N > > , time : IndexVec < N , Time > , }
}