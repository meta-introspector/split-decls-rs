use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: in_inclusive_range");
# [inline (always)] fn in_inclusive_range (i : usize , start : usize , end : usize) -> bool { i . wrapping_sub (start) <= (end - start) }
}