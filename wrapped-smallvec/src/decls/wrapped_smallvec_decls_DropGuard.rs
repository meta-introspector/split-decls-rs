use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct DropGuard<T> {
    ptr: *mut T,
    len: usize,
}
