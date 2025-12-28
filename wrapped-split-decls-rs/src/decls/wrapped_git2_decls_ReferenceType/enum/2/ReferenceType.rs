use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An enumeration of all possible kinds of references."] # [derive (PartialEq , Eq , Copy , Clone , Debug)] pub enum ReferenceType { # [doc = " A reference which points at an object id."] Direct , # [doc = " A reference which points at another reference."] Symbolic , }