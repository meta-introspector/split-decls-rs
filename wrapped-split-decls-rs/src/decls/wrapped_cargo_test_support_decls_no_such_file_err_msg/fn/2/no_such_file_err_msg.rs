use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: no_such_file_err_msg");
# [doc = " The error message for ENOENT."] pub fn no_such_file_err_msg () -> String { std :: io :: Error :: from_raw_os_error (2) . to_string () }
}