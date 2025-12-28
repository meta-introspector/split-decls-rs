use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: update_expect");
fn update_expect () -> bool { env :: var ("UPDATE_EXPECT") . is_ok () }
}