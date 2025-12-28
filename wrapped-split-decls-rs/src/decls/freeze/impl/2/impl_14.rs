use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T > FreezeWriteGuard < 'a , T > { pub fn freeze (self) -> & 'a T { self . frozen . store (true , Ordering :: Release) ; unsafe { & * self . data . as_ptr () } } }