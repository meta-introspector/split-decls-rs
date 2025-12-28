use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct DropCounter < 'a > { count : & 'a Cell < u32 > , }