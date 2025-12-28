use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for RustSourceWorkspaceConfig { fn default () -> Self { RustSourceWorkspaceConfig :: default_cargo () } }
}