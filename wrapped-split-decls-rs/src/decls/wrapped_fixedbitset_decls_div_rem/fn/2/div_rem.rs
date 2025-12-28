use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: div_rem");
# [inline] fn div_rem (x : usize , denominator : usize) -> (usize , usize) { (x / denominator , x % denominator) }
}