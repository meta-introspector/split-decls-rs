use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Contains the rayon thread pool configuration. Use [`ThreadPoolBuilder`] instead."] # [doc = ""] # [doc = " [`ThreadPoolBuilder`]: struct.ThreadPoolBuilder.html"] # [deprecated (note = "Use `ThreadPoolBuilder`")] # [derive (Default)] pub struct Configuration { builder : ThreadPoolBuilder , }
}