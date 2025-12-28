use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Converts a quantity of kilobytes to bytes."] pub fn kb (size : impl Into < u64 >) -> u64 { size . into () * KB }