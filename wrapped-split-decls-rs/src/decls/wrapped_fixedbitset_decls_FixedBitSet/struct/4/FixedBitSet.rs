use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " `FixedBitSet` is a simple fixed size set of bits that each can"] # [doc = " be enabled (1 / **true**) or disabled (0 / **false**)."] # [doc = ""] # [doc = " The bit set has a fixed capacity in terms of enabling bits (and the"] # [doc = " capacity can grow using the `grow` method)."] # [doc = ""] # [doc = " Derived traits depend on both the zeros and ones, so [0,1] is not equal to"] # [doc = " [0,1,0]."] # [derive (Debug , Eq)] pub struct FixedBitSet { pub (crate) data : NonNull < MaybeUninit < SimdBlock > > , capacity : usize , # [doc = " length in bits"] pub (crate) length : usize , }
}