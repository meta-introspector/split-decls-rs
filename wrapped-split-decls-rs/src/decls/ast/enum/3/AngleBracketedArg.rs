use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Either an argument for a generic parameter or a constraint on an associated item."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum AngleBracketedArg { # [doc = " A generic argument for a generic parameter."] Arg (GenericArg) , # [doc = " A constraint on an associated item."] Constraint (AssocItemConstraint) , }