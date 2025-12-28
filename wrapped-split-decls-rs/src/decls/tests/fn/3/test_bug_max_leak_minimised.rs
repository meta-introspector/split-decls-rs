use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_bug_max_leak_minimised () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (3 , 0) , (3 , 4) , (4 , 3)]) ; let mut annotations = Maxes (IndexVec :: new () , | w | match w { 4 => 1 , _ => 0 , }) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . annotation (sccs . scc (2)) , 0) ; assert_eq ! (annotations . annotation (sccs . scc (3)) , 1) ; assert_eq ! (annotations . annotation (sccs . scc (0)) , 1) ; }