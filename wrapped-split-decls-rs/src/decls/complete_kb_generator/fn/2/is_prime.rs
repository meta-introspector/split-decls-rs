use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn is_prime (n : i32) -> bool { if n < 2 { return false ; } if n == 2 { return true ; } if n % 2 == 0 { return false ; } let sqrt_n = (n as f64) . sqrt () as i32 ; for i in (3 ..= sqrt_n) . step_by (2) { if n % i == 0 { return false ; } } true }
}