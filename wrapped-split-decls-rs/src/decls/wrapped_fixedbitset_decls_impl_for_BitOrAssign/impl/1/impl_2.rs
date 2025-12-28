use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl BitOrAssign for FixedBitSet { fn bitor_assign (& mut self , other : Self) { self . union_with (& other) ; } }