use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator producing elements in the union of two sets."] # [doc = ""] # [doc = " This struct is created by the [`FixedBitSet::union`] method."] pub struct Union < 'a > { iter : Chain < Ones < 'a > , Difference < 'a > > , }
}