use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct LifetimeParamId { pub parent : GenericDefId , pub local_id : LocalLifetimeParamId , }
}