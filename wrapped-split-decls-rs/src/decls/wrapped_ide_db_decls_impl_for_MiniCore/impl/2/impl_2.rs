use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > MiniCore < 'a > { # [inline] pub fn new (minicore : & 'a str) -> Self { Self (minicore) } # [inline] pub const fn default () -> Self { Self (test_utils :: MiniCore :: RAW_SOURCE) } }
}