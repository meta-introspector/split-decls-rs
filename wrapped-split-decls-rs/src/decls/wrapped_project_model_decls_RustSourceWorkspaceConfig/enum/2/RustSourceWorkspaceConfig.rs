use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Debug , PartialEq , Eq)] pub enum RustSourceWorkspaceConfig { CargoMetadata (CargoMetadataConfig) , Json (ProjectJson) , }
}