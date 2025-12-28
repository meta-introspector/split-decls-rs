use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Default)] pub struct RwLock < T > (parking_lot :: RwLock < T >) ;