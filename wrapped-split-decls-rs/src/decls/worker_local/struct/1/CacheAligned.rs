use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [repr (align (64))] # [derive (Debug)] struct CacheAligned < T > (T) ;