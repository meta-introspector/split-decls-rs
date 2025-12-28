use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Determines towards which end of a struct layout optimizations will try to place the best niches."] enum NicheBias { Start , End , }
}