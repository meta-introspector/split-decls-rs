use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator producing elements in the difference of two sets."] # [doc = ""] # [doc = " This struct is created by the [`FixedBitSet::difference`] method."] pub struct Difference < 'a > { iter : Ones < 'a > , other : & 'a FixedBitSet , }