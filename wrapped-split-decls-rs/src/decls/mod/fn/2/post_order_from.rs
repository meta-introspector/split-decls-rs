use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn post_order_from < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node ,) -> Vec < G :: Node > { post_order_from_to (graph , start_node , None) }
}