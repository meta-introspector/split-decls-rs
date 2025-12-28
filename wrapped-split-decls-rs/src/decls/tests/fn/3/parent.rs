use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn parent () { let pairs = vec ! [(2 , 0) , (2 , 2) , (0 , 0) , (0 , 0) , (1 , 0) , (1 , 1) , (3 , 0) , (3 , 3) , (4 , 0) , (4 , 1) , (1 , 3) ,] ; let mut relation = TransitiveRelationBuilder :: default () ; for (a , b) in pairs { relation . add (a , b) ; } let relation = relation . freeze () ; let p = relation . postdom_parent (3) ; assert_eq ! (p , Some (0)) ; }