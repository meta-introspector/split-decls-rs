use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , Debug)] pub enum RangeOrOffset { Range (TextRange) , Offset (TextSize) , }