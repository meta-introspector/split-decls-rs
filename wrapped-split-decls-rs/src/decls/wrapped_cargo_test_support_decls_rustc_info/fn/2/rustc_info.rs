use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn rustc_info () -> & 'static RustcInfo { static RUSTC_INFO : OnceLock < RustcInfo > = OnceLock :: new () ; RUSTC_INFO . get_or_init (RustcInfo :: new) }
}