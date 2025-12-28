use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn unextend_sign (val : i64 , nbytes : usize) -> u64 { let shift = (8 - nbytes) * 8 ; (val << shift) as u64 >> shift }