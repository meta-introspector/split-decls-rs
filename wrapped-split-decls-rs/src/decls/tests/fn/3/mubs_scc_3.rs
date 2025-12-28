use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn mubs_scc_3 () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("a" , "c") ; relation . add ("c" , "d") ; relation . add ("d" , "e") ; relation . add ("e" , "c") ; relation . add ("b" , "d") ; relation . add ("b" , "e") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_upper_bounds ("a" , "b") , vec ! ["c"]) ; }
}