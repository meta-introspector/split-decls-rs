use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cfg (unix)] fn overflow_code () -> Option < i32 > { None }
}