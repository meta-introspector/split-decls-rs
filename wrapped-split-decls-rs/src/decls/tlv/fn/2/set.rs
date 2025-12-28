use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Sets the current thread-local value"] # [inline] pub (crate) fn set (value : Tlv) { TLV . with (| tlv | tlv . set (value . 0)) ; }
}