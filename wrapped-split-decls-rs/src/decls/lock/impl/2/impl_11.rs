use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : 'a > Deref for LockGuard < 'a , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . lock . data . get () } } }