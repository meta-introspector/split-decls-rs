use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn is_cfg (attr : & ast :: Attribute) -> bool { attr . has_name (rustc_span :: symbol :: sym :: cfg) }
}