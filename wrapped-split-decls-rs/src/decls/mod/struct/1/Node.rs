use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct Node < N > { first_edge : [EdgeIndex ; 2] , pub data : N , }
}