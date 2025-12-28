use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_insert_presorted_at_end () { let mut map = SortedMap :: new () ; map . insert (1 , 1) ; map . insert (2 , 2) ; map . insert_presorted (vec ! [(3 , 3) , (8 , 8)]) ; let expected = vec ! [(1 , 1) , (2 , 2) , (3 , 3) , (8 , 8)] ; assert_eq ! (elements (map) , expected) ; }
}