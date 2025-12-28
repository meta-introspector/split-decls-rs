use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum ProjectManifest { ProjectJson (ManifestPath) , CargoToml (ManifestPath) , CargoScript (ManifestPath) , }
}