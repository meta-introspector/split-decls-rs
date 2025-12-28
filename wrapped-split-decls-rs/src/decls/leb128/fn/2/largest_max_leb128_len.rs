use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Returns the length of the longest LEB128 encoding of all supported integer types."] pub const fn largest_max_leb128_len () -> usize { max_leb128_len :: < u128 > () }