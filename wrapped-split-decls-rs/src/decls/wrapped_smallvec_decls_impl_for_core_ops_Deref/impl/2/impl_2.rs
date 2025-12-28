use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , const N : usize > core :: ops :: Deref for SmallVec < T , N > { type Target = [T] ; # [inline] fn deref (& self) -> & Self :: Target { self . as_slice () } }