use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_i32 () { check_round_trip (vec ! [- 1 , 2 , - 3 , i32 :: MIN , 0 , 1 , i32 :: MAX , 2 , 1]) ; }
}