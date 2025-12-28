use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_u8 () { let mut vec = vec ! [] ; for i in u8 :: MIN .. u8 :: MAX { vec . push (i) ; } check_round_trip (vec) ; }