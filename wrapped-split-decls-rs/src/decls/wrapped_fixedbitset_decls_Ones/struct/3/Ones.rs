use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An  iterator producing the indices of the set bit in a set."] # [doc = ""] # [doc = " This struct is created by the [`FixedBitSet::ones`] method."] # [derive (Clone)] pub struct Ones < 'a > { bitset_front : usize , bitset_back : usize , block_idx_front : usize , block_idx_back : usize , remaining_blocks : core :: slice :: Iter < 'a , usize > , }
}