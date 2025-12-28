use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn with_elements_standard (elements : & [usize] , domain_size : usize) -> DenseBitSet < usize > { let mut s = DenseBitSet :: new_empty (domain_size) ; for & e in elements { assert ! (s . insert (e)) ; } s }
}