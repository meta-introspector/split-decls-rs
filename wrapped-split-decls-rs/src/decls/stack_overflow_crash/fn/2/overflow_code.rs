use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (windows)] fn overflow_code () -> Option < i32 > { use std :: os :: windows :: process :: ExitStatusExt ; ExitStatus :: from_raw (0xc00000fd) . code () }