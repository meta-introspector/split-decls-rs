use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn diamond () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3)]) ; let sccs : UsizeSccs = Sccs :: new (& graph) ; assert_eq ! (sccs . num_sccs () , 4) ; assert_eq ! (sccs . num_sccs () , 4) ; }