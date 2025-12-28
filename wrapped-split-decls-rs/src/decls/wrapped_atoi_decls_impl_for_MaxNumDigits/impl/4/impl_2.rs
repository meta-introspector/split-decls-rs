use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I > MaxNumDigits for I where I : Bounded + Zero + DivAssign + Ord + Copy , { # [doc = " Returns the maximum number of digits a nonnegative representation of `I` can have depending"] # [doc = " on `radix`."] fn max_num_digits (radix : I) -> usize { let mut max = I :: max_value () ; let mut d = 0 ; while max > I :: zero () { d += 1 ; max /= radix ; } d } # [doc = " Returns the maximum number of digits a negative representation of `I` can have depending"] # [doc = " on `radix`."] fn max_num_digits_negative (radix : I) -> usize { let mut min = I :: min_value () ; let mut d = 0 ; while min < I :: zero () { d += 1 ; min /= radix ; } d } }
}