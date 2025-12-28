use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct InstantiatedVariant < 'db > { pub (crate) inner : Variant , pub (crate) args : GenericArgs < 'db > , }