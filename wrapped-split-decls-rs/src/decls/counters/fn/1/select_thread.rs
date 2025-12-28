use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn select_thread (word : usize , shift : usize) -> usize { (word >> shift) & THREADS_MAX }