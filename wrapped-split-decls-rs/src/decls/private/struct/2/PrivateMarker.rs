use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " If this type is pub but not publicly reachable, third parties"] # [doc = " can't name it and can't implement traits using it."] # [allow (missing_debug_implementations)] pub struct PrivateMarker ;