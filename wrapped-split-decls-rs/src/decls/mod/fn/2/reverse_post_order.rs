use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn reverse_post_order < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node ,) -> Vec < G :: Node > { let mut vec = post_order_from (graph , start_node) ; vec . reverse () ; vec }