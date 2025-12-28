use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_double_cycle_max () { let graph = TestGraph :: new (0 , & [(0 , 1) , (1 , 2) , (1 , 4) , (2 , 3) , (2 , 4) , (3 , 5) , (4 , 1) , (5 , 4)]) ; let mut annotations = Maxes (IndexVec :: new () , | n | if n == 5 { 2 } else { 1 }) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . 0 [sccs . scc (0)] . 0 , 2) ; }
}