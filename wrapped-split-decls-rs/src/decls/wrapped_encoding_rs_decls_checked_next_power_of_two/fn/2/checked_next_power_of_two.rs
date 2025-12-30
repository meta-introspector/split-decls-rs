use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: checked_next_power_of_two");
# [cfg (feature = "alloc")] # [inline (always)] fn checked_next_power_of_two (opt : Option < usize >) -> Option < usize > { opt . map (| n | n . next_power_of_two ()) }
}