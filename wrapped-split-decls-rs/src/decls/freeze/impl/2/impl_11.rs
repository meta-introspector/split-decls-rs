use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T : ? Sized + 'a > Deref for FreezeReadGuard < 'a , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . data . as_ptr () } } }
}