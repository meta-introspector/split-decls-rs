use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Uuid { # [inline] fn default () -> Self { Uuid :: nil () } }
}