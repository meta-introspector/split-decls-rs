use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn mubs_scc_4 () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("a" , "c") ; relation . add ("c" , "d") ; relation . add ("d" , "e") ; relation . add ("e" , "c") ; relation . add ("a" , "d") ; relation . add ("b" , "e") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_upper_bounds ("a" , "b") , vec ! ["c"]) ; }