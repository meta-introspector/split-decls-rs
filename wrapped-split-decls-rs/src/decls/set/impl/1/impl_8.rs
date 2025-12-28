use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Default for SsoHashSet < T > { # [inline] fn default () -> Self { Self :: new () } }
}