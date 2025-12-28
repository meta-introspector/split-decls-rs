use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Contains the rayon thread pool configuration. Use [`ThreadPoolBuilder`] instead."] # [deprecated (note = "Use `ThreadPoolBuilder`")] # [derive (Default)] pub struct Configuration { builder : ThreadPoolBuilder , }