use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] pub struct InjectionPoint { pub location : String , pub bott_level : BottLevel , pub introspection_data : IntrospectionData , }
}