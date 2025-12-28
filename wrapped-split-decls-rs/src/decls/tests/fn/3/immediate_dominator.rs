use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn immediate_dominator () { let graph = TestGraph :: new (1 , & [(1 , 2) , (2 , 3)]) ; let d = dominators (& graph) ; assert_eq ! (d . immediate_dominator (0) , None) ; assert_eq ! (d . immediate_dominator (1) , None) ; assert_eq ! (d . immediate_dominator (2) , Some (1)) ; assert_eq ! (d . immediate_dominator (3) , Some (2)) ; }