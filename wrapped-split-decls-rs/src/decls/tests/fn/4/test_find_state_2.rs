use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_find_state_2");
# [test] fn test_find_state_2 () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 4) , (1 , 2) , (1 , 3) , (2 , 1) , (3 , 0) , (4 , 2)]) ; let sccs : UsizeSccs = Sccs :: new (& graph) ; assert_eq ! (sccs . num_sccs () , 1) ; assert_eq ! (sccs . scc (0) , 0) ; assert_eq ! (sccs . scc (1) , 0) ; assert_eq ! (sccs . scc (2) , 0) ; assert_eq ! (sccs . scc (3) , 0) ; assert_eq ! (sccs . scc (4) , 0) ; assert_eq ! (sccs . successors (0) , & [] as & [usize]) ; }
}