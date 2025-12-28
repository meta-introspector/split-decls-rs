use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: select_thread");
# [inline] fn select_thread (word : usize , shift : usize) -> usize { (word >> shift) & THREADS_MAX }
}