use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > Default for MiniCore < 'a > { # [inline] fn default () -> Self { Self :: default () } }
}