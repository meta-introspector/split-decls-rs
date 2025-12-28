use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_usize");
# [test] fn test_usize () { check_round_trip (vec ! [1 , 2 , 3 , usize :: MIN , 0 , 1 , usize :: MAX , 2 , 1]) ; }
}