use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_insert_presorted_last_elem_equal () { let mut map = SortedMap :: new () ; map . insert (2 , 2) ; map . insert (8 , 8) ; map . insert_presorted (vec ! [(3 , 3) , (8 , 0)]) ; let expected = vec ! [(2 , 2) , (3 , 3) , (8 , 0)] ; assert_eq ! (elements (map) , expected) ; }
}