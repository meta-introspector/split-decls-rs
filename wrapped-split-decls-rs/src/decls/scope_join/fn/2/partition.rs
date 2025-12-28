use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn partition < T : PartialOrd + Send > (v : & mut [T]) -> usize { let pivot = v . len () - 1 ; let mut i = 0 ; for j in 0 .. pivot { if v [j] <= v [pivot] { v . swap (i , j) ; i += 1 ; } } v . swap (i , pivot) ; i }
}