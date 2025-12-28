use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: last_set_before_adjacent");
# [test] fn last_set_before_adjacent () { let mut set = IntervalSet :: < u32 > :: new (300) ; set . insert_range (0 .. 3) ; set . insert_range (3 .. 5) ; assert_eq ! (set . last_set_in (0 .. 3) , Some (2)) ; assert_eq ! (set . last_set_in (0 .. 5) , Some (4)) ; assert_eq ! (set . last_set_in (3 .. 5) , Some (4)) ; set . insert_range (2 .. 5) ; assert_eq ! (set . last_set_in (0 .. 3) , Some (2)) ; assert_eq ! (set . last_set_in (0 .. 5) , Some (4)) ; assert_eq ! (set . last_set_in (3 .. 5) , Some (4)) ; }
}