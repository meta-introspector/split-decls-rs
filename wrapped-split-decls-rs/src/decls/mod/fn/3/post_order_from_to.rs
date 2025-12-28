use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn post_order_from_to < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node , end_node : Option < G :: Node > ,) -> Vec < G :: Node > { let mut visited : IndexVec < G :: Node , bool > = IndexVec :: from_elem_n (false , graph . num_nodes ()) ; let mut result : Vec < G :: Node > = Vec :: with_capacity (graph . num_nodes ()) ; if let Some (end_node) = end_node { visited [end_node] = true ; } post_order_walk (graph , start_node , & mut result , & mut visited) ; result }