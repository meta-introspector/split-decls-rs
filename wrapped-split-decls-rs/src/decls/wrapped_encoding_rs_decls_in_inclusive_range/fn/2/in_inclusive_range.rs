use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline (always)] fn in_inclusive_range (i : usize , start : usize , end : usize) -> bool { i . wrapping_sub (start) <= (end - start) }