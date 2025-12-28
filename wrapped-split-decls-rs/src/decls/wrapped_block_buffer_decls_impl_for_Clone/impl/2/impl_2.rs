use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < BS : ArraySize , K : BufferKind > Clone for BlockBuffer < BS , K > { # [inline] fn clone (& self) -> Self { unsafe { ptr :: read (self) } } }
}