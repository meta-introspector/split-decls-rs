use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait Successors : DirectedGraph { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > ; }
}