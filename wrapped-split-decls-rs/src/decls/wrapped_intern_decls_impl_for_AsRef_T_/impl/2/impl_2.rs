use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Internable + ? Sized > AsRef < T > for Interned < T > { # [inline] fn as_ref (& self) -> & T { & self . arc } }
}