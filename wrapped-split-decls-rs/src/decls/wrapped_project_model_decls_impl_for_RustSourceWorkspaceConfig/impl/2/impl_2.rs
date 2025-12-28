use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl RustSourceWorkspaceConfig { pub fn default_cargo () -> Self { RustSourceWorkspaceConfig :: CargoMetadata (Default :: default ()) } }
}