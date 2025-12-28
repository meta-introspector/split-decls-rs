use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Test that one can emulate join with `scope`:"] fn pseudo_join < F , G > (f : F , g : G) where F : FnOnce () + Send , G : FnOnce () + Send , { rustc_thread_pool :: scope (| s | { s . spawn (| _ | g ()) ; f () ; }) ; }