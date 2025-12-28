use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A type which allows mutation using a lock until"] # [doc = " the value is frozen and can be accessed lock-free."] # [doc = ""] # [doc = " Unlike `RwLock`, it can be used to prevent mutation past a point."] # [derive (Default)] pub struct FreezeLock < T > { data : UnsafeCell < T > , frozen : AtomicBool , # [doc = " This lock protects writes to the `data` and `frozen` fields."] lock : RwLock < () > , }
}