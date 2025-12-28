use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
pub const INVALID_EDGE_INDEX : EdgeIndex = EdgeIndex (usize :: MAX) ;
}