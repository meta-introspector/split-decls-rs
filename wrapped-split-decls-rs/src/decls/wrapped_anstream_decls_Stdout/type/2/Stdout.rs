use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An adaptive wrapper around the global standard output stream of the current process"] pub type Stdout = AutoStream < std :: io :: Stdout > ;