use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Borrow < [u8] > for OwnedSlice { # [inline] fn borrow (& self) -> & [u8] { self } }
}