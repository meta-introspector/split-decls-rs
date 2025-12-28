use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: analysis_is_send");
# [test] fn analysis_is_send () { fn is_send < T : Send > () { } is_send :: < Analysis > () ; }
}