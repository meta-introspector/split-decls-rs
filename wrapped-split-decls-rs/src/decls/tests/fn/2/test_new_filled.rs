use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_new_filled () { for i in 0 .. 128 { let idx_buf = DenseBitSet :: new_filled (i) ; let elems : Vec < usize > = idx_buf . iter () . collect () ; let expected : Vec < usize > = (0 .. i) . collect () ; assert_eq ! (elems , expected) ; } }
}