use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [cfg (any (miri , target_arch = "wasm32"))] pub struct MmapMut (Vec < u8 >) ;
}