use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : 'a > DerefMut for LockGuard < 'a , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . lock . data . get () } } }