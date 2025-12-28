use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct InstantiatedEnum < 'db > { pub (crate) inner : Enum , pub (crate) args : GenericArgs < 'db > , }