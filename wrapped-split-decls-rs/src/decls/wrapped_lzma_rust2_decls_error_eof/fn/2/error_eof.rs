use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: error_eof");
# [cfg (not (feature = "std"))] # [inline (always)] fn error_eof () -> Error { Error :: Eof }
}