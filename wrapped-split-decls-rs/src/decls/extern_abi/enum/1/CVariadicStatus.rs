use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "nightly")] pub enum CVariadicStatus { NotSupported , Stable , Unstable { feature : Symbol } , }