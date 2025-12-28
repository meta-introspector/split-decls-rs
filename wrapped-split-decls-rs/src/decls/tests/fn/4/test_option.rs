use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_option () { check_round_trip (vec ! [Some (- 1i8)]) ; check_round_trip (vec ! [Some (- 2i16)]) ; check_round_trip (vec ! [Some (- 3i32)]) ; check_round_trip (vec ! [Some (- 4i64)]) ; check_round_trip (vec ! [Some (- 5isize)]) ; let none_i8 : Option < i8 > = None ; check_round_trip (vec ! [none_i8]) ; let none_i16 : Option < i16 > = None ; check_round_trip (vec ! [none_i16]) ; let none_i32 : Option < i32 > = None ; check_round_trip (vec ! [none_i32]) ; let none_i64 : Option < i64 > = None ; check_round_trip (vec ! [none_i64]) ; let none_isize : Option < isize > = None ; check_round_trip (vec ! [none_isize]) ; }
}