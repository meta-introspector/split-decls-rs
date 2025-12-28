use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_bug_minimised");
# [test] fn test_bug_minimised () { let graph = TestGraph :: new (0 , & [(0 , 3) , (0 , 1) , (3 , 2) , (2 , 3) , (1 , 4) , (4 , 5) , (5 , 4)]) ; let mut annotations = Maxes (IndexVec :: new () , | n | match n { 3 => 1 , _ => 0 , }) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . annotation (sccs . scc (2)) , 1) ; assert_eq ! (annotations . annotation (sccs . scc (1)) , 0) ; assert_eq ! (annotations . annotation (sccs . scc (4)) , 0) ; }
}