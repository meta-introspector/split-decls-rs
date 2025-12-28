use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Lock < T > { # [inline (always)] # [track_caller] pub fn with_lock < F : FnOnce (& mut T) -> R , R > (& self , f : F) -> R { f (& mut * self . lock ()) } # [inline (always)] # [track_caller] pub fn borrow (& self) -> LockGuard < '_ , T > { self . lock () } # [inline (always)] # [track_caller] pub fn borrow_mut (& self) -> LockGuard < '_ , T > { self . lock () } }
}