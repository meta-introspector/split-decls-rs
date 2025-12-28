use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < BODY > Job for HeapJob < BODY > where BODY : FnOnce (JobRefId) + Send , { unsafe fn execute (this : * const ()) { let pointer = this . expose_provenance () ; let this = unsafe { Box :: from_raw (this as * mut Self) } ; tlv :: set (this . tlv) ; (this . job) (JobRefId { pointer }) ; } }
}