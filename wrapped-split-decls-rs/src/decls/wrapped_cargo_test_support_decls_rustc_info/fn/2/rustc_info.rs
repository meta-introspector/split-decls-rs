use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: rustc_info");
fn rustc_info () -> & 'static RustcInfo { static RUSTC_INFO : OnceLock < RustcInfo > = OnceLock :: new () ; RUSTC_INFO . get_or_init (RustcInfo :: new) }
}