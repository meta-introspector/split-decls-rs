use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator producing elements in the intersection of two sets."] # [doc = ""] # [doc = " This struct is created by the [`FixedBitSet::intersection`] method."] pub struct Intersection < 'a > { iter : Ones < 'a > , other : & 'a FixedBitSet , }