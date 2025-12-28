use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_precise_algorithm () { assert_ne ! (edit_distance ("ab" , "ba" , usize :: MAX) , Some (2)) ; assert_ne ! (edit_distance ("abde" , "bcaed" , usize :: MAX) , Some (3)) ; assert_eq ! (edit_distance ("abde" , "bcaed" , usize :: MAX) , Some (4)) ; }
}