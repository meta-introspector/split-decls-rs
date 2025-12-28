use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ExternAbi { # [doc = " Default ABI chosen for `extern fn` declarations without an explicit ABI."] pub const FALLBACK : ExternAbi = ExternAbi :: C { unwind : false } ; pub fn name (self) -> & 'static str { self . as_str () } }
}