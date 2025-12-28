use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default)] pub struct RdfStateMachine { pub triples : Vec < RdfTriple > , pub execution_state : ExecutionState , }
}