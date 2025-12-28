use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc (hidden)] pub trait AnyExpectations : Any + Send + Sync { }