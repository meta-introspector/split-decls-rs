use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn limits () { assert_eq ! (Ok (u128 :: MAX) , u128 :: from_str_radix (& u128 :: MAX . to_base (36) , 36)) ; assert_eq ! (Ok (u64 :: MAX) , u64 :: from_str_radix (& u64 :: MAX . to_base (36) , 36)) ; assert_eq ! (Ok (u32 :: MAX) , u32 :: from_str_radix (& u32 :: MAX . to_base (36) , 36)) ; }
}