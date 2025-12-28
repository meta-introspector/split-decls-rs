use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "std")] # [doc = " A convenience trait that can be used together with the type aliases defined to"] # [doc = " get access to the `new()` and `with_capacity()` methods for the HashSet type aliases."] pub trait HashSetExt { # [doc = " Constructs a new HashSet"] fn new () -> Self ; # [doc = " Constructs a new HashSet with a given initial capacity"] fn with_capacity (capacity : usize) -> Self ; }