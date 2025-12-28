use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_hash_map () { use std :: collections :: HashMap ; let mut map = HashMap :: new () ; for i in - 100i64 .. 100i64 { map . insert (i * 100000 , i * 10000) ; } check_round_trip (vec ! [map]) ; }
}