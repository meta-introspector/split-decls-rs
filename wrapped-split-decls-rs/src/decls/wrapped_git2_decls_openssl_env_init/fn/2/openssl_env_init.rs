use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: openssl_env_init");
# [cfg (any (windows , target_os = "macos" , target_os = "ios" , not (feature = "https")))] fn openssl_env_init () { }
}