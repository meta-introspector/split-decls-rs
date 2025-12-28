use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: copy_error");
# [cfg (not (feature = "std"))] # [inline (always)] fn copy_error (error : & Error) -> Error { * error }
}