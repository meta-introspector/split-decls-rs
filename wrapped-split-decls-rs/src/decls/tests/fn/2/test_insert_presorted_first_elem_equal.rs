use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_insert_presorted_first_elem_equal () { let mut map = SortedMap :: new () ; map . insert (2 , 2) ; map . insert (8 , 8) ; map . insert_presorted (vec ! [(2 , 0) , (7 , 7)]) ; let expected = vec ! [(2 , 0) , (7 , 7) , (8 , 8)] ; assert_eq ! (elements (map) , expected) ; }
}