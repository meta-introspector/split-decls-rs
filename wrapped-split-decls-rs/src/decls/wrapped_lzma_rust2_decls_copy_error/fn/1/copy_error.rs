use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (not (feature = "std"))] # [inline (always)] fn copy_error (error : & Error) -> Error { * error }