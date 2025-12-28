use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cfg (not (feature = "std"))] # [inline (always)] fn error_eof () -> Error { Error :: Eof }
}