use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_range");
# [test] fn test_range () { let mut map = SortedMap :: new () ; map . insert (1 , 1) ; map . insert (3 , 3) ; map . insert (6 , 6) ; map . insert (9 , 9) ; let keys = | s : & [(_ , _)] | s . into_iter () . map (| e | e . 0) . collect :: < Vec < u32 > > () ; for start in 0 .. 11 { for end in 0 .. 11 { if end < start { continue ; } let mut expected = vec ! [1 , 3 , 6 , 9] ; expected . retain (| & x | x >= start && x < end) ; assert_eq ! (keys (map . range (start .. end)) , expected , "range = {}..{}" , start , end) ; } } }
}