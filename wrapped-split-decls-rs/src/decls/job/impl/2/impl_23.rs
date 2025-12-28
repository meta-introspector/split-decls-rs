use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < BODY > Job for ArcJob < BODY > where BODY : Fn (JobRefId) + Send + Sync , { unsafe fn execute (this : * const ()) { let pointer = this . expose_provenance () ; let this = unsafe { Arc :: from_raw (this as * mut Self) } ; (this . job) (JobRefId { pointer }) ; } }
}