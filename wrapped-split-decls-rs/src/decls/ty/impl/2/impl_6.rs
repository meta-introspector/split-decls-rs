use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FieldIdx { # [doc = " The second field, at index 1."] # [doc = ""] # [doc = " For use alongside [`FieldIdx::ZERO`], particularly with scalar pairs."] pub const ONE : FieldIdx = FieldIdx :: from_u32 (1) ; }