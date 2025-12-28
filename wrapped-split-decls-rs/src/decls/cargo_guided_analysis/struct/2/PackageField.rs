use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct PackageField { pub name : String , pub version : String , pub source : String , pub dependencies : Vec < String > , pub checksum : Option < String > , }
}