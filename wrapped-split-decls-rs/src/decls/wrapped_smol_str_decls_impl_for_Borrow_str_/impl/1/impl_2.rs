use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Borrow < str > for SmolStr { # [inline (always)] fn borrow (& self) -> & str { self . as_str () } }
}