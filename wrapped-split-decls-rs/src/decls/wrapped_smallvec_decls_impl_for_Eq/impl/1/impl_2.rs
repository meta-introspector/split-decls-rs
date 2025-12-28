use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , const N : usize > Eq for SmallVec < T , N > where T : Eq { }