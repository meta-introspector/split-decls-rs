use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] pub fn make_hash < K : Hash + ? Sized > (val : & K) -> u64 { let mut state = FxHasher :: default () ; val . hash (& mut state) ; state . finish () }
}