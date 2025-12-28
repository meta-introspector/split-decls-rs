use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn mubs_intermediate_node_on_one_side_only () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("a" , "c") ; relation . add ("c" , "d") ; relation . add ("b" , "d") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_upper_bounds ("a" , "b") , vec ! ["d"]) ; }