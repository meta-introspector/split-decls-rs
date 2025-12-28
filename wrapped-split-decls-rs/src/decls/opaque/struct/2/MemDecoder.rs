use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct MemDecoder < 'a > { start : * const u8 , current : * const u8 , end : * const u8 , _marker : PhantomData < & 'a u8 > , }
}