use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: is_unpin");
# [cfg (feature = "io-util")] # [cfg (test)] fn is_unpin < T : Unpin > () { }
}