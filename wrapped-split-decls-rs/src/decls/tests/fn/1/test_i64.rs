use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_i64 () { check_round_trip (vec ! [- 1 , 2 , - 3 , i64 :: MIN , 0 , 1 , i64 :: MAX , 2 , 1]) ; }
}