use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: capacity_overflow");
# [cold] fn capacity_overflow () -> ! { panic ! ("capacity overflow") }
}