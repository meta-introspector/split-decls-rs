use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline (always)] fn align_down (val : usize , align : usize) -> usize { debug_assert ! (align . is_power_of_two ()) ; val & ! (align - 1) }
}