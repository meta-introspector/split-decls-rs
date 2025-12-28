use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl BitAndAssign < & Self > for FixedBitSet { fn bitand_assign (& mut self , other : & Self) { self . intersect_with (other) ; } }