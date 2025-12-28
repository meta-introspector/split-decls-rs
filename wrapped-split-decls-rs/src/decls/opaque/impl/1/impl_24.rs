use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl IntEncodedWithFixedSize { pub const ENCODED_SIZE : usize = 8 ; }
}