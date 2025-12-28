use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [cfg (not (feature = "utf8"))] pub type DefaultCharAccumulator = AsciiParser ;
}