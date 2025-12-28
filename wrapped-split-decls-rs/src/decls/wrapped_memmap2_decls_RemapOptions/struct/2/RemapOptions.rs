use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Options for [`Mmap::remap`] and [`MmapMut::remap`]."] # [derive (Copy , Clone , Default , Debug)] # [cfg (target_os = "linux")] pub struct RemapOptions { may_move : bool , }