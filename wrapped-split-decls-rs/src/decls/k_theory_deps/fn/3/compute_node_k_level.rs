use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn compute_node_k_level (node : usize , graph : & DependencyGraph) -> usize { let in_degree = graph . reverse . get (& node) . map_or (0 , | v | v . len ()) ; let out_degree = graph . forward . get (& node) . map_or (0 , | v | v . len ()) ; match (in_degree , out_degree) { (0 , 0) => 0 , (0 , _) => 1 , (_ , 0) => 2 , (1 , 1) => 3 , (1 , _) | (_ , 1) => 4 , (2 , 2) => 5 , (_ , _) if in_degree + out_degree > 10 => 7 , _ => 6 , } }
}