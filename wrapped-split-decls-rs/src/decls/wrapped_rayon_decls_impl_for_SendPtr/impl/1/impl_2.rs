use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > SendPtr < T > { fn get (self) -> * mut T { self . 0 } }
}