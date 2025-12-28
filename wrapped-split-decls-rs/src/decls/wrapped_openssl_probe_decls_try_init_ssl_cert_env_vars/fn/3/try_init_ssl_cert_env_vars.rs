use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: try_init_ssl_cert_env_vars");
# [doc = " Deprecated as this isn't sound, use [`try_init_openssl_env_vars`] instead."] # [doc (hidden)] # [deprecated (note = "use try_init_openssl_env_vars instead, this function is not safe")] pub fn try_init_ssl_cert_env_vars () -> bool { unsafe { try_init_openssl_env_vars () } }
}