use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Lazy block buffer."] pub type LazyBuffer < B > = BlockBuffer < B , Lazy > ;