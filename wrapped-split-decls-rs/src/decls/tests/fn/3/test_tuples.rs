use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_tuples () { check_round_trip (vec ! [('x' , () , false , 5u32)]) ; check_round_trip (vec ! [(9i8 , 10u16 , 15i64)]) ; check_round_trip (vec ! [(- 12i16 , 11u8 , 12usize)]) ; check_round_trip (vec ! [(1234567isize , 100000000000000u64 , 99999999999999i64)]) ; check_round_trip (vec ! [(String :: new () , "some string" . to_string ())]) ; }