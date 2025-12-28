use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_u16 () { for i in [u16 :: MIN , 111 , 3333 , 55555 , u16 :: MAX] { check_round_trip (vec ! [1 , 2 , 3 , i , i , i]) ; } }