use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A guard holding mutable access to a `FreezeLock` which is in a locked state or frozen."] # [must_use = "if unused the FreezeLock may immediately unlock"] pub struct FreezeWriteGuard < 'a , T : ? Sized > { _lock_guard : WriteGuard < 'a , () > , frozen : & 'a AtomicBool , data : NonNull < T > , marker : PhantomData < & 'a mut T > , }
}