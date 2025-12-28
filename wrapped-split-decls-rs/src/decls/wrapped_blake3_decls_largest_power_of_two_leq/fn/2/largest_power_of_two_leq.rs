use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: largest_power_of_two_leq");
fn largest_power_of_two_leq (n : usize) -> usize { ((n / 2) + 1) . next_power_of_two () }
}