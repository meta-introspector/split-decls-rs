use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Default > Default for Sharded < T > { # [inline] fn default () -> Self { Self :: new (T :: default) } }
}