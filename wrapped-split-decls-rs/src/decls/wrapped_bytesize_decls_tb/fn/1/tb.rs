use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Converts a quantity of terabytes to bytes."] pub fn tb < V : Into < u64 > > (size : V) -> u64 { size . into () * TB }