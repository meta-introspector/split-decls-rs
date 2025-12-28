use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait IndexedVal { fn to_val (index : usize) -> Self ; fn to_index (& self) -> usize ; }
}