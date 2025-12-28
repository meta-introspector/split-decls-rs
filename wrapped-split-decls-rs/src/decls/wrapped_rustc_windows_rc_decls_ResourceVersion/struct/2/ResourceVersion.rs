use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Windows resources store versions as four 16-bit integers."] struct ResourceVersion { major : u16 , minor : u16 , patch : u16 , build : u16 , }