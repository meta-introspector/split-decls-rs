use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl SleepData { # [doc = " Checks if the conditions for a deadlock holds and if so calls the deadlock handler"] # [inline] pub (super) fn deadlock_check (& self , deadlock_handler : & Option < Box < DeadlockHandler > >) { if self . active_threads == 0 && self . blocked_threads > 0 { (deadlock_handler . as_ref () . unwrap ()) () ; } } }
}