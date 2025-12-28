use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator that moves out of a `ThreadLocal`."] # [derive (Debug)] pub struct IntoIter < T : Send > { thread_local : ThreadLocal < T > , raw : RawIter , }