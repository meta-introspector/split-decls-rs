use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > DerefMut for MaybeUninitSlice < 'a > { fn deref_mut (& mut self) -> & mut [MaybeUninit < u8 >] { self . 0 . as_mut_slice () } }
}