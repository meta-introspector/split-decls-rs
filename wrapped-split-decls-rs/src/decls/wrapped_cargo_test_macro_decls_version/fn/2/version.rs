use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: version");
fn version () -> (u32 , bool) { LazyLock :: force (& VERSION) . clone () }
}