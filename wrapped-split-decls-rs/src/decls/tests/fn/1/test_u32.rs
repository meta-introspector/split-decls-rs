use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_u32 () { check_round_trip (vec ! [1 , 2 , 3 , u32 :: MIN , 0 , 1 , u32 :: MAX , 2 , 1]) ; }