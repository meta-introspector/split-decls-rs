use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_offset_keys () { let mut map = SortedMap :: new () ; map . insert (1 , 1) ; map . insert (3 , 3) ; map . insert (6 , 6) ; map . offset_keys (| k | * k += 1) ; let mut expected = SortedMap :: new () ; expected . insert (2 , 1) ; expected . insert (4 , 3) ; expected . insert (7 , 6) ; assert_eq ! (map , expected) ; }