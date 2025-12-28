use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Runs code with the hardware DIT feature or equivalent enabled when possible."] # [cfg (any (not (target_arch = "aarch64") , miri))] # [inline (always)] pub (crate) fn with_dit < T , F > (f : F) -> T where F : FnOnce () -> T , { f () }