use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Float { pub fn size (self) -> Size { use Float :: * ; match self { F16 => Size :: from_bits (16) , F32 => Size :: from_bits (32) , F64 => Size :: from_bits (64) , F128 => Size :: from_bits (128) , } } pub fn align < C : HasDataLayout > (self , cx : & C) -> AbiAlign { use Float :: * ; let dl = cx . data_layout () ; AbiAlign :: new (match self { F16 => dl . f16_align , F32 => dl . f32_align , F64 => dl . f64_align , F128 => dl . f128_align , }) } }
}