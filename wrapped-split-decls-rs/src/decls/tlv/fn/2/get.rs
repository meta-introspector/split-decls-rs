use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Returns the current thread-local value"] # [inline] pub (crate) fn get () -> Tlv { TLV . with (| tlv | Tlv (tlv . get ())) }
}