use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn extend_sign128 (val : u128 , nbytes : usize) -> i128 { let shift = (16 - nbytes) * 8 ; (val << shift) as i128 >> shift }
}