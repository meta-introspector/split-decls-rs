use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Describes the number of vertices discovered at the time when processing of a particular vertex"] # [doc = " started and when it finished. Both values are zero for unreachable vertices."] # [derive (Copy , Clone , Default , Debug)] struct Time { start : u32 , finish : u32 , }