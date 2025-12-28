use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_insert_presorted_shuffle () { let mut map = SortedMap :: new () ; map . insert (2 , 2) ; map . insert (7 , 7) ; map . insert_presorted (vec ! [(1 , 1) , (3 , 3) , (8 , 8)]) ; let expected = vec ! [(1 , 1) , (2 , 2) , (3 , 3) , (7 , 7) , (8 , 8)] ; assert_eq ! (elements (map) , expected) ; }