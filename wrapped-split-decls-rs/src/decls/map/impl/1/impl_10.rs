use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K , V > Default for SsoHashMap < K , V > { # [inline] fn default () -> Self { Self :: new () } }
}