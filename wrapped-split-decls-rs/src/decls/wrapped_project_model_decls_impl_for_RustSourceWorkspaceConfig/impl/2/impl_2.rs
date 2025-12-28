use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl RustSourceWorkspaceConfig { pub fn default_cargo () -> Self { RustSourceWorkspaceConfig :: CargoMetadata (Default :: default ()) } }