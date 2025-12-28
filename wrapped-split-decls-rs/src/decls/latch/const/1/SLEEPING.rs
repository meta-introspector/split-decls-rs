use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Latch is not set, owning thread is asleep on this latch and"] # [doc = " must be awoken."] const SLEEPING : usize = 2 ;