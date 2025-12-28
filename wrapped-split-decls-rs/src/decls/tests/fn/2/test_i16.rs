use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_i16 () { for i in [i16 :: MIN , - 100 , 0 , 101 , i16 :: MAX] { check_round_trip (vec ! [- 1 , 2 , - 3 , i , i , i , 2]) ; } }