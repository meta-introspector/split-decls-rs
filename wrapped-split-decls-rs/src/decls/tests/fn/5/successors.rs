use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn successors () { let graph = create_graph () ; assert_eq ! (graph . successors (0) , & [1]) ; assert_eq ! (graph . successors (1) , & [2 , 3]) ; assert_eq ! (graph . successors (2) , & [] as & [usize]) ; assert_eq ! (graph . successors (3) , & [4]) ; assert_eq ! (graph . successors (4) , & [] as & [usize]) ; assert_eq ! (graph . successors (5) , & [1]) ; assert_eq ! (graph . successors (6) , & [] as & [usize]) ; let graph = create_graph_with_back_refs () ; assert_eq ! (graph . successors (0) , & [1]) ; assert_eq ! (graph . successors (1) , & [2 , 3]) ; assert_eq ! (graph . successors (2) , & [] as & [usize]) ; assert_eq ! (graph . successors (3) , & [4]) ; assert_eq ! (graph . successors (4) , & [] as & [usize]) ; assert_eq ! (graph . successors (5) , & [1]) ; assert_eq ! (graph . successors (6) , & [] as & [usize]) ; }
}