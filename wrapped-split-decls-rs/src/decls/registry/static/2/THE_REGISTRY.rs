use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " ////////////////////////////////////////////////////////////////////////"] # [doc = " Initialization"] static mut THE_REGISTRY : Option < Arc < Registry > > = None ;