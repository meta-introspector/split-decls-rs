use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: char_has_case");
# [must_use] pub const fn char_has_case (c : char) -> bool { c . is_lowercase () || c . is_uppercase () }
}