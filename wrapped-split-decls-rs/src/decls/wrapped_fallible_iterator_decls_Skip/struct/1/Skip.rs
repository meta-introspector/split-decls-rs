use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator which skips initial elements."] # [derive (Clone , Debug)] pub struct Skip < I > { it : I , n : usize , }