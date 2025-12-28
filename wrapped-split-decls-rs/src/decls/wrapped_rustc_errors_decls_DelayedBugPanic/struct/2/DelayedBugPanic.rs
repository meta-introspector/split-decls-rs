use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Signifies that the compiler died due to a delayed bug rather than a failed"] # [doc = " assertion, etc."] pub struct DelayedBugPanic ;