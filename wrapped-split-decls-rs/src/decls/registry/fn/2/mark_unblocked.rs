use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Mark a previously blocked Rayon worker thread as unblocked"] # [inline] pub fn mark_unblocked (registry : & Registry) { registry . sleep . mark_unblocked () }
}