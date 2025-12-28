use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Access the thread-local generator"] # [doc = ""] # [doc = " Use [`rand::rng()`](rng()) instead."] # [cfg (feature = "thread_rng")] # [deprecated (since = "0.9.0" , note = "Renamed to `rng`")] # [inline] pub fn thread_rng () -> crate :: rngs :: ThreadRng { rng () }
}