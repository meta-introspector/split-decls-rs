use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn div_rem (x : usize , denominator : usize) -> (usize , usize) { (x / denominator , x % denominator) }
}