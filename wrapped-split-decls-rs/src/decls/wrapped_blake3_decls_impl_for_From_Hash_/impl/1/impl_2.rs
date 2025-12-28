use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Hash > for [u8 ; OUT_LEN] { # [inline] fn from (hash : Hash) -> Self { hash . 0 } }
}