use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: to_upper_snake_case");
pub fn to_upper_snake_case (s : & str) -> String { to_snake_case (s , char :: to_uppercase) }
}