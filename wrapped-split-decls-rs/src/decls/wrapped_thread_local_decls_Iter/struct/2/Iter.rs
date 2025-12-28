use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Iterator over the contents of a `ThreadLocal`."] # [derive (Debug)] pub struct Iter < 'a , T : Send + Sync > { thread_local : & 'a ThreadLocal < T > , raw : RawIter , }
}