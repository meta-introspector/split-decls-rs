use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: check_error_send_sync");
# [test] fn check_error_send_sync () { _send_sync :: < ThreadPoolBuildError > () ; }
}