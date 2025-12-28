use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: to_c_string");
pub fn to_c_string (s : & str) -> * const c_char { CString :: new (s) . expect ("CString::new failed") . into_raw () }
}