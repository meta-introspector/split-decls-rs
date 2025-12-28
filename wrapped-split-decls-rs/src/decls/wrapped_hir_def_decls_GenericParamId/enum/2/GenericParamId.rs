use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A generic param"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum GenericParamId { TypeParamId (TypeParamId) , ConstParamId (ConstParamId) , LifetimeParamId (LifetimeParamId) , }