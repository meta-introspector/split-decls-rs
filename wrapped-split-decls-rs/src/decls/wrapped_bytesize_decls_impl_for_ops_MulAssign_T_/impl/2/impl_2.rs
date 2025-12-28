use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > ops :: MulAssign < T > for ByteSize where T : Into < u64 > , { # [inline (always)] fn mul_assign (& mut self , rhs : T) { self . 0 *= rhs . into () ; } }
}