use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A guard holding shared access to a `FreezeLock` which is in a locked state or frozen."] # [must_use = "if unused the FreezeLock may immediately unlock"] pub struct FreezeReadGuard < 'a , T : ? Sized > { _lock_guard : Option < ReadGuard < 'a , () > > , data : NonNull < T > , }