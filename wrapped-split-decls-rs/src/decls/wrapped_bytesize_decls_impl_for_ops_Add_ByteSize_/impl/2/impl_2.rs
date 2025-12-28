use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ops :: Add < ByteSize > for ByteSize { type Output = ByteSize ; # [inline (always)] fn add (self , rhs : ByteSize) -> ByteSize { ByteSize (self . 0 + rhs . 0) } }
}