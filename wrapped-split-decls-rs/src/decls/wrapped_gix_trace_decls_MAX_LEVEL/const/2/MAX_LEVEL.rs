use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The maximum allowed level for tracing items, as compiled in."] # [cfg (not (feature = "tracing-detail"))] pub const MAX_LEVEL : Level = Level :: Coarse ;