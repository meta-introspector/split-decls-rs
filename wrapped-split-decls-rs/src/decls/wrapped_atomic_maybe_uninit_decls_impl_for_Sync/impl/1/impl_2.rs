use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T : Primitive > Sync for AtomicMaybeUninit < T > { }
}