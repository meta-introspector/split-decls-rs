use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A pointer to the `RegistryData` which uniquely identifies a registry."] # [doc = " This identifier can be reused if the registry gets freed."] # [derive (Clone , Copy , PartialEq)] struct RegistryId (* const RegistryData) ;