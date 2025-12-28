use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Debug , Copy , Eq , PartialEq , HashStable_Generic)] pub enum AllocatorKind { Global , Default , }