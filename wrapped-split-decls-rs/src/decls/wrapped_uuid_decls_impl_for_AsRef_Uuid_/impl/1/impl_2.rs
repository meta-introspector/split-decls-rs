use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsRef < Uuid > for Uuid { # [inline] fn as_ref (& self) -> & Uuid { self } }
}