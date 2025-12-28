use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn file_contains (file : & SourceFile , pos : BytePos) -> bool { file . contains (pos) && ! file . is_empty () }