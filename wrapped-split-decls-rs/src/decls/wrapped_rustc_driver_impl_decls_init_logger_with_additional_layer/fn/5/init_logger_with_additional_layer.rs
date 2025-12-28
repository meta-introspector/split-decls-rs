use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " This allows tools to enable rust logging without having to magically match rustc's"] # [doc = " tracing crate version. In contrast to `init_rustc_env_logger`, it allows you to"] # [doc = " choose the logger config directly rather than having to set an environment variable."] # [doc = " Moreover, in contrast to `init_logger`, it allows you to add a custom tracing layer"] # [doc = " via `build_subscriber`, for example `|| Registry::default().with(custom_layer)`."] pub fn init_logger_with_additional_layer < F , T > (early_dcx : & EarlyDiagCtxt , cfg : rustc_log :: LoggerConfig , build_subscriber : F ,) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { if let Err (error) = rustc_log :: init_logger_with_additional_layer (cfg , build_subscriber) { early_dcx . early_fatal (error . to_string ()) ; } }
}