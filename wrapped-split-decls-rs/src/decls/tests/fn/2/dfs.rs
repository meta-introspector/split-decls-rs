use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn dfs () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3) , (3 , 0)]) ; let result : Vec < usize > = DepthFirstSearch :: new (& graph) . with_start_node (0) . collect () ; assert_eq ! (result , vec ! [0 , 2 , 3 , 1]) ; }
}