use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: opaque");
pub fn opaque < T : Debug > (value : & T) -> Opaque { Opaque (format ! ("{value:?}")) }
}