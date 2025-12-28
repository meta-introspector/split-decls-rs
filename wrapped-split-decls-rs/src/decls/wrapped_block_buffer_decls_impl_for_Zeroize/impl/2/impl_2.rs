use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "zeroize")] impl < BS : ArraySize , K : BufferKind > Zeroize for BlockBuffer < BS , K > { # [inline] fn zeroize (& mut self) { self . buffer . zeroize () ; self . pos . zeroize () ; } }