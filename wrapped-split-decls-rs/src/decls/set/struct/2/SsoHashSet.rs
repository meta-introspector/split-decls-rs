use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Small-storage-optimized implementation of a set."] # [doc = ""] # [doc = " Stores elements in a small array up to a certain length"] # [doc = " and switches to `HashSet` when that length is exceeded."] # [derive (Clone)] pub struct SsoHashSet < T > { map : SsoHashMap < T , () > , }
}