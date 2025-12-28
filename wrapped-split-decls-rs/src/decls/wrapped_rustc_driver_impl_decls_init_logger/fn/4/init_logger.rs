use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: init_logger");
# [doc = " This allows tools to enable rust logging without having to magically match rustc's"] # [doc = " tracing crate version. In contrast to `init_rustc_env_logger` it allows you to choose"] # [doc = " the logger config directly rather than having to set an environment variable."] pub fn init_logger (early_dcx : & EarlyDiagCtxt , cfg : rustc_log :: LoggerConfig) { if let Err (error) = rustc_log :: init_logger (cfg) { early_dcx . early_fatal (error . to_string ()) ; } }
}