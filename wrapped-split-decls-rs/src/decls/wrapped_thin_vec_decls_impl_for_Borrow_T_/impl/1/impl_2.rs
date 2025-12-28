use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Borrow < [T] > for ThinVec < T > { fn borrow (& self) -> & [T] { self . as_slice () } }
}