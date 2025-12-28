use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : ? Sized + 'a > DerefMut for FreezeWriteGuard < 'a , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . data . as_ptr () } } }