use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Scales { fn default () -> Self { Scales :: SI () } }
}