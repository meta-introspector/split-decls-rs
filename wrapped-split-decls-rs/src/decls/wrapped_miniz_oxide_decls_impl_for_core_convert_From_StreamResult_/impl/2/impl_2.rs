use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (not (feature = "rustc-dep-of-std"))] impl core :: convert :: From < & StreamResult > for MZResult { fn from (res : & StreamResult) -> Self { res . status } }