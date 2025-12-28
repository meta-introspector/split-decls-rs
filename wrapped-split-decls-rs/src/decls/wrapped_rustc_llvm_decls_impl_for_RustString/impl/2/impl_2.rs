use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl RustString { pub fn build_byte_buffer (closure : impl FnOnce (& Self)) -> Vec < u8 > { let buf = RustStringInner :: default () ; closure (buf . as_opaque ()) ; buf . into_inner () } }
}