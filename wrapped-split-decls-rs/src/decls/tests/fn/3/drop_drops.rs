use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn drop_drops () { let flag = Arc :: new (AtomicBool :: new (false)) ; let flag_prime = Arc :: clone (& flag) ; let d = defer (move | | flag_prime . store (true , atomic :: Ordering :: Relaxed)) ; let slice = slice_owned (d , | _ | & []) ; assert_eq ! (flag . load (atomic :: Ordering :: Relaxed) , false) ; drop (slice) ; assert_eq ! (flag . load (atomic :: Ordering :: Relaxed) , true) ; }
}