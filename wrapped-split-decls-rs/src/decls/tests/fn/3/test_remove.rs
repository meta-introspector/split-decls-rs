use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_remove () { let mut map = SortedMap :: new () ; let mut expected = Vec :: new () ; for x in 0 .. 10 { map . insert (x , x) ; expected . push ((x , x)) ; } for x in 0 .. 10 { let mut map = map . clone () ; let mut expected = expected . clone () ; assert_eq ! (map . remove (& x) , Some (x)) ; expected . remove (x as usize) ; assert_eq ! (map . iter () . cloned () . collect ::< Vec < _ >> () , expected) ; } }