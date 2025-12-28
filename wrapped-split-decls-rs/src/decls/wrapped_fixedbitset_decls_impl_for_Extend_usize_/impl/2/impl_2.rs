use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Sets the bit at index **i** to **true** for each item **i** in the input **src**."] impl Extend < usize > for FixedBitSet { fn extend < I : IntoIterator < Item = usize > > (& mut self , src : I) { let iter = src . into_iter () ; for i in iter { if i >= self . len () { self . grow (i + 1) ; } self . put (i) ; } } }