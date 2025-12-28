use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Eager block buffer."] pub type EagerBuffer < B > = BlockBuffer < B , Eager > ;