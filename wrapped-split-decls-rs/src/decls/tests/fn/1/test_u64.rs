use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_u64 () { check_round_trip (vec ! [1 , 2 , 3 , u64 :: MIN , 0 , 1 , u64 :: MAX , 2 , 1]) ; }