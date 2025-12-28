use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Debug , PartialEq , Eq)] pub struct Adjustment < 'db > { pub source : Type < 'db > , pub target : Type < 'db > , pub kind : Adjust , }