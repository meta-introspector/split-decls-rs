use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline (always)] fn in_range32 (i : u32 , start : u32 , end : u32) -> bool { i . wrapping_sub (start) < (end - start) }