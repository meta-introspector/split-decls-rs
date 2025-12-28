use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (not (feature = "std"))] # [inline (always)] fn error_invalid_data (msg : & 'static str) -> Error { Error :: InvalidData (msg) }