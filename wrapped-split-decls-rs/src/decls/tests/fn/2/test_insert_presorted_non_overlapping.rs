use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_insert_presorted_non_overlapping () { let mut map = SortedMap :: new () ; map . insert (2 , 0) ; map . insert (8 , 0) ; map . insert_presorted (vec ! [(3 , 0) , (7 , 0)]) ; let expected = vec ! [2 , 3 , 7 , 8] ; assert_eq ! (keys (map) , expected) ; }