use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Converts a quantity of kibibytes to bytes."] pub fn kib < V : Into < u64 > > (size : V) -> u64 { size . into () * KIB }