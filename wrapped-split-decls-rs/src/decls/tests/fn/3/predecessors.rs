use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn predecessors () { let graph = create_graph_with_back_refs () ; assert_eq ! (graph . predecessors (0) , & []) ; assert_eq ! (graph . predecessors (1) , & [0 , 5]) ; assert_eq ! (graph . predecessors (2) , & [1]) ; assert_eq ! (graph . predecessors (3) , & [1]) ; assert_eq ! (graph . predecessors (4) , & [3]) ; assert_eq ! (graph . predecessors (5) , & []) ; assert_eq ! (graph . predecessors (6) , & []) ; }
}