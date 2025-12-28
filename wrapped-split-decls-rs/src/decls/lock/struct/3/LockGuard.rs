use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A guard holding mutable access to a `Lock` which is in a locked state."] # [must_use = "if unused the Lock will immediately unlock"] pub struct LockGuard < 'a , T > { lock : & 'a Lock < T > , marker : PhantomData < & 'a mut T > , # [doc = " The synchronization mode of the lock. This is explicitly passed to let LLVM relate it"] # [doc = " to the original lock operation."] mode : Mode , }