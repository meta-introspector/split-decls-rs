use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " The maximum supported number of lanes in a SIMD vector."] # [doc = ""] # [doc = " This value is selected based on backend support:"] # [doc = " * LLVM does not appear to have a vector width limit."] # [doc = " * Cranelift stores the base-2 log of the lane count in a 4 bit integer."] pub const MAX_SIMD_LANES : u64 = 1 << 0xF ;
}