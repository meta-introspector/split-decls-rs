use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct SparseMatrix { size : usize , connections : Vec < (usize , usize , f64) > , global_nodes : Vec < usize > , }
}