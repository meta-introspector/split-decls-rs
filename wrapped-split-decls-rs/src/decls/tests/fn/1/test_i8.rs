use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_i8 () { let mut vec = vec ! [] ; for i in i8 :: MIN .. i8 :: MAX { vec . push (i) ; } check_round_trip (vec) ; }