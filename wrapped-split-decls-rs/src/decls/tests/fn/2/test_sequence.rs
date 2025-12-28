use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_sequence () { let mut vec = vec ! [] ; for i in - 100i64 .. 100i64 { vec . push (i * 100000) ; } check_round_trip (vec ! [vec]) ; }