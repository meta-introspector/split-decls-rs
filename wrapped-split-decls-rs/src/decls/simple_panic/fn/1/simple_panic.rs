use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [should_panic (expected = "should panic")] fn simple_panic () { join (| | { } , | | panic ! ("should panic")) ; }