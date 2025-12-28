use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Signifies that the compiler died with an explicit call to `.bug`"] # [doc = " or `.span_bug` rather than a failed assertion, etc."] pub struct ExplicitBug ;