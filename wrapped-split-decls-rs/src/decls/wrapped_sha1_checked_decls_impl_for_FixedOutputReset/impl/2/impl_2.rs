use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FixedOutputReset for Sha1 { # [inline] fn finalize_into_reset (& mut self , out : & mut Output < Self >) { self . finalize_inner (out) ; Reset :: reset (self) ; } }
}