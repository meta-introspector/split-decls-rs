use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An adaptive wrapper around the global standard error stream of the current process"] pub type Stderr = AutoStream < std :: io :: Stderr > ;