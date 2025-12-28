use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct DependencyMetadata { pub name : String , pub value : Value , pub default_features : Option < bool > , pub features : Vec < String > , pub optional : bool , }