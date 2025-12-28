use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Dependency { fn default () -> Self { Dependency :: Version (String :: new ()) } }
}