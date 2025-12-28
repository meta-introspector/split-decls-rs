use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The result type used by `process_obligation`."] # [repr (C)] # [derive (Debug)] pub enum ProcessResult < O , E > { Unchanged , Changed (ThinVec < O >) , Error (E) , }
}