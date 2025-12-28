use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Debug)] pub enum AbiFromStrErr { # [doc = " not a known ABI"] Unknown , # [doc = " no \"-unwind\" variant can be used here"] NoExplicitUnwind , }