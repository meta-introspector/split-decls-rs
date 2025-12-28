use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MinMaxes { fn annotation (& self , scc : usize) -> MinMaxIn { self . 0 [scc] } }
}