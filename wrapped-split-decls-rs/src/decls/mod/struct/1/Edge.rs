use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct Edge < E > { next_edge : [EdgeIndex ; 2] , source : NodeIndex , target : NodeIndex , pub data : E , }