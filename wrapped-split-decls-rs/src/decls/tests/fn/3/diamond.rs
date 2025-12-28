use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn diamond () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3)]) ; let d = dominators (& graph) ; assert_eq ! (d . immediate_dominator (0) , None) ; assert_eq ! (d . immediate_dominator (1) , Some (0)) ; assert_eq ! (d . immediate_dominator (2) , Some (0)) ; assert_eq ! (d . immediate_dominator (3) , Some (0)) ; }
}