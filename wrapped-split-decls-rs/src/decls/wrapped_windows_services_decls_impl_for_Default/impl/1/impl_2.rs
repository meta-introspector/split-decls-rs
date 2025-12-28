use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Service < '_ > { fn default () -> Self { Self :: new () } }
}