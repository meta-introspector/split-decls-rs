use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_char () { let vec = vec ! ['a' , 'b' , 'c' , 'd' , 'A' , 'X' , ' ' , '#' , 'Ö' , 'Ä' , 'µ' , '€'] ; check_round_trip (vec) ; }
}