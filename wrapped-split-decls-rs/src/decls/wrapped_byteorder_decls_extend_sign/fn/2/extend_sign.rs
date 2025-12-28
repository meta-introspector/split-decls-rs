use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: extend_sign");
# [inline] fn extend_sign (val : u64 , nbytes : usize) -> i64 { let shift = (8 - nbytes) * 8 ; (val << shift) as i64 >> shift }
}