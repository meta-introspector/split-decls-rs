use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const RC_BIT_MODEL_OFFSET : u32 = (1u32 << MOVE_BITS) . wrapping_sub (1) . wrapping_sub (BIT_MODEL_TOTAL) ;