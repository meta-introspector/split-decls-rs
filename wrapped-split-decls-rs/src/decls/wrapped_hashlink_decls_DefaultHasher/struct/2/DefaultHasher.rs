use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Default hasher, as selected by hashbrown."] # [derive (Clone)] pub struct DefaultHasher (< hashbrown :: DefaultHashBuilder as BuildHasher > :: Hasher) ;