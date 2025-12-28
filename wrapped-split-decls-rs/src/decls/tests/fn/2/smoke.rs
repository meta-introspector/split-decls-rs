use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn smoke () { let slice = slice_owned (vec ! [1 , 2 , 3 , 4 , 5 , 6] , Vec :: as_slice) ; assert_eq ! (&* slice , [1 , 2 , 3 , 4 , 5 , 6]) ; }
}