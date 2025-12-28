use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: read");
fn read (path : & AbsPath) -> Option < Vec < u8 > > { std :: fs :: read (path) . ok () }
}