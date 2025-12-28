use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ops :: SubAssign < ByteSize > for ByteSize { # [inline (always)] fn sub_assign (& mut self , rhs : ByteSize) { self . 0 -= rhs . 0 ; } }
}