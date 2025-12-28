use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_uniq");
# [test] fn test_uniq () { let s1 = S (1) ; let s2 = S (2) ; let s3 = S (3) ; let s4 = S (1) ; let v1 = Interned :: new_unchecked (& s1) ; let v2 = Interned :: new_unchecked (& s2) ; let v3a = Interned :: new_unchecked (& s3) ; let v3b = Interned :: new_unchecked (& s3) ; let v4 = Interned :: new_unchecked (& s4) ; assert_ne ! (v1 , v2) ; assert_ne ! (v2 , v3a) ; assert_eq ! (v1 , v1) ; assert_eq ! (v3a , v3b) ; assert_ne ! (v1 , v4) ; assert_eq ! (v1 . cmp (& v2) , Ordering :: Less) ; assert_eq ! (v3a . cmp (& v2) , Ordering :: Greater) ; assert_eq ! (v1 . cmp (& v1) , Ordering :: Equal) ; assert_eq ! (v3a . cmp (& v3b) , Ordering :: Equal) ; assert_eq ! (v1 . partial_cmp (& v2) , Some (Ordering :: Less)) ; assert_eq ! (v3a . partial_cmp (& v2) , Some (Ordering :: Greater)) ; assert_eq ! (v1 . partial_cmp (& v1) , Some (Ordering :: Equal)) ; assert_eq ! (v3a . partial_cmp (& v3b) , Some (Ordering :: Equal)) ; }
}