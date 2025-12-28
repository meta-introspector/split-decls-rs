use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_three_sccs");
# [test] fn test_three_sccs () { let graph = TestGraph :: new (0 , & [(0 , 1) , (1 , 2) , (2 , 1) , (3 , 2)]) ; let sccs : UsizeSccs = Sccs :: new (& graph) ; assert_eq ! (sccs . num_sccs () , 3) ; assert_eq ! (sccs . scc (0) , 1) ; assert_eq ! (sccs . scc (1) , 0) ; assert_eq ! (sccs . scc (2) , 0) ; assert_eq ! (sccs . scc (3) , 2) ; assert_eq ! (sccs . successors (0) , & [] as & [usize]) ; assert_eq ! (sccs . successors (1) , & [0]) ; assert_eq ! (sccs . successors (2) , & [0]) ; }
}