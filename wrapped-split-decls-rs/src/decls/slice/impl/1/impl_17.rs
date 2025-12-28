use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < I : Idx , T > Send for IndexSlice < I , T > where T : Send { }