use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Converts a quantity of pebibytes to bytes."] pub fn pib < V : Into < u64 > > (size : V) -> u64 { size . into () * PIB }