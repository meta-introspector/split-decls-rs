use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_hash_bit_matrix () { use rustc_index :: bit_set :: BitMatrix ; let a : BitMatrix < usize , usize > = BitMatrix :: new (1 , 1) ; let b : BitMatrix < usize , usize > = BitMatrix :: new (1 , 2) ; assert_ne ! (a , b) ; assert_ne ! (hash (& a) , hash (& b)) ; }
}