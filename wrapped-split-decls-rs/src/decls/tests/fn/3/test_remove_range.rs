use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_remove_range () { let mut map = SortedMap :: new () ; map . insert (1 , 1) ; map . insert (3 , 3) ; map . insert (6 , 6) ; map . insert (9 , 9) ; for start in 0 .. 11 { for end in 0 .. 11 { if end < start { continue ; } let mut expected = vec ! [1 , 3 , 6 , 9] ; expected . retain (| & x | x < start || x >= end) ; let mut map = map . clone () ; map . remove_range (start .. end) ; assert_eq ! (keys (map) , expected , "range = {}..{}" , start , end) ; } } }
}