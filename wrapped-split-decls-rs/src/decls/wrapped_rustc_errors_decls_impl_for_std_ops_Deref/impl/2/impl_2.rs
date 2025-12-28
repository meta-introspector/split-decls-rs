use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > std :: ops :: Deref for DiagCtxtHandle < 'a > { type Target = & 'a DiagCtxt ; fn deref (& self) -> & Self :: Target { & self . dcx } }
}