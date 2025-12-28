use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Debug , Copy , Eq , PartialEq , HashStable_Generic)] pub enum AllocatorKind { Global , Default , }
}