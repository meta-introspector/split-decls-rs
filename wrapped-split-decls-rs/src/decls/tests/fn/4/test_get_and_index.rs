use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_get_and_index");
# [test] fn test_get_and_index () { let mut map = SortedMap :: new () ; let mut expected = Vec :: new () ; for x in 0 .. 100 { let x = 1000 - x ; if x & 1 == 0 { map . insert (x , x) ; } expected . push (x) ; } for mut x in expected { if x & 1 == 0 { assert_eq ! (map . get (& x) , Some (& x)) ; assert_eq ! (map . get_mut (& x) , Some (& mut x)) ; assert_eq ! (map [& x] , x) ; assert_eq ! (& mut map [& x] , & mut x) ; } else { assert_eq ! (map . get (& x) , None) ; assert_eq ! (map . get_mut (& x) , None) ; } } }
}