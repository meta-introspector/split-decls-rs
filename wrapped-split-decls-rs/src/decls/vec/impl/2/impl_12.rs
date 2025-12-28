use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T > DerefMut for IndexVec < I , T > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }