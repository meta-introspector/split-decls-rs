use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct Slot < V > { value : V , index_and_lock : AtomicU32 , }