use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_isize");
# [test] fn test_isize () { check_round_trip (vec ! [- 1 , 2 , - 3 , isize :: MIN , 0 , 1 , isize :: MAX , 2 , 1]) ; }
}