use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : 'a > Drop for LockGuard < 'a , T > { # [inline] fn drop (& mut self) { match self . mode { Mode :: NoSync => { let cell = unsafe { & self . lock . mode_union . no_sync } ; debug_assert ! (cell . get ()) ; cell . set (false) ; } Mode :: Sync => unsafe { self . lock . mode_union . sync . unlock () } , } } }