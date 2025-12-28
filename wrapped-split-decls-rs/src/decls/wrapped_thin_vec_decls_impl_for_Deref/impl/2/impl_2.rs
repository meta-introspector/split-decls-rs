use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "gecko-ffi")] impl < T , const N : usize > Deref for AutoThinVec < T , N > { type Target = ThinVec < T > ; fn deref (& self) -> & Self :: Target { & self . inner } }
}