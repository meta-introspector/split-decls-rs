use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub (super) struct RegistryId { addr : usize , }
}