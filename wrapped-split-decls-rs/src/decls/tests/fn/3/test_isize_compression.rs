use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_isize_compression () { fn check_hash (a : u64 , b : u64) { let hash_a = hash (& (a as isize , b as isize)) ; let hash_b = hash (& (b as isize , a as isize)) ; assert_ne ! (hash_a , hash_b , "The hash stayed the same when permuting values `{a}` and `{b}`!" ,) ; } check_hash (0xAA , 0xAAAA) ; check_hash (0xFF , 0xFFFF) ; check_hash (0xAAAA , 0xAAAAAA) ; check_hash (0xAAAAAA , 0xAAAAAAAA) ; check_hash (0xFF , 0xFFFFFFFFFFFFFFFF) ; check_hash (u64 :: MAX , 1) ; }
}