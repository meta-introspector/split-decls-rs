use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn dfs_debug () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3) , (3 , 0)]) ; let mut dfs = DepthFirstSearch :: new (& graph) . with_start_node (0) ; dfs . complete_search () ; assert_eq ! (format ! ("{{0, 1, 2, 3}}") , format ! ("{:?}" , dfs)) ; }
}