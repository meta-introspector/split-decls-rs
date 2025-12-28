use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cfg (feature = "alloc")] # [track_caller] # [inline (always)] # [cfg (not (debug_assertions))] unsafe fn assume (v : bool) { if ! v { unsafe { core :: hint :: unreachable_unchecked () ; } } }
}