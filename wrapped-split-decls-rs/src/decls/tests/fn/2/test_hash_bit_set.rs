use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_hash_bit_set () { use rustc_index :: bit_set :: DenseBitSet ; let a : DenseBitSet < usize > = DenseBitSet :: new_empty (1) ; let b : DenseBitSet < usize > = DenseBitSet :: new_empty (2) ; assert_ne ! (a , b) ; assert_ne ! (hash (& a) , hash (& b)) ; }