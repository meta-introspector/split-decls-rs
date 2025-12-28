use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cfg (feature = "serde")] fn is_true (b : & bool) -> bool { * b }
}