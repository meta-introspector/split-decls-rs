use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Converts a quantity of mebibytes to bytes."] pub fn mib < V : Into < u64 > > (size : V) -> u64 { size . into () * MIB }