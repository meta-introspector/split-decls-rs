use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_insert_and_iter () { let mut map = SortedMap :: new () ; let mut expected = Vec :: new () ; for x in 0 .. 100 { assert_eq ! (map . iter () . cloned () . collect ::< Vec < _ >> () , expected) ; let x = 1000 - x * 2 ; map . insert (x , x) ; expected . insert (0 , (x , x)) ; } }