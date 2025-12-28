use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_edit_distance_limit");
# [test] fn test_edit_distance_limit () { assert_eq ! (edit_distance ("abc" , "abcd" , 1) , Some (1)) ; assert_eq ! (edit_distance ("abc" , "abcd" , 0) , None) ; assert_eq ! (edit_distance ("abc" , "xyz" , 3) , Some (3)) ; assert_eq ! (edit_distance ("abc" , "xyz" , 2) , None) ; }
}