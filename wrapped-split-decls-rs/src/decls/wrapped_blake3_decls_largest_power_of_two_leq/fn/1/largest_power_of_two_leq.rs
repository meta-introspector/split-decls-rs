use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn largest_power_of_two_leq (n : usize) -> usize { ((n / 2) + 1) . next_power_of_two () }