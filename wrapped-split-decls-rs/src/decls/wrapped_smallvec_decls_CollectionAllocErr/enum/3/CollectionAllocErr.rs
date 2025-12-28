use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Error type for APIs with fallible heap allocation"] # [derive (Debug)] pub enum CollectionAllocErr { # [doc = " Overflow `usize::MAX` or other error during size computation"] CapacityOverflow , # [doc = " The allocator return an error"] AllocErr { # [doc = " The layout that was passed to the allocator"] layout : Layout , } , }
}