use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct PrincipalComponent { pub name : String , pub variance_explained : f64 , pub key_features : Vec < String > , }