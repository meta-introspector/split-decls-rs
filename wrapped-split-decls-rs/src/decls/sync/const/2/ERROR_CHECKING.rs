use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " This makes locks panic if they are already held."] # [doc = " It is only useful when you are running in a single thread"] const ERROR_CHECKING : bool = false ;