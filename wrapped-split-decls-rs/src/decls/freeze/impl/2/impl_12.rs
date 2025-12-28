use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T : ? Sized > FreezeReadGuard < 'a , T > { # [inline] pub fn map < U : ? Sized > (this : Self , f : impl FnOnce (& T) -> & U) -> FreezeReadGuard < 'a , U > { FreezeReadGuard { data : NonNull :: from (f (& * this)) , _lock_guard : this . _lock_guard } } }
}