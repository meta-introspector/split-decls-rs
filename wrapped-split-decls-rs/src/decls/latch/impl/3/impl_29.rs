use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Latch for CountLatch { # [inline] unsafe fn set (this : * const Self) { if unsafe { (* this) . counter . fetch_sub (1 , Ordering :: SeqCst) == 1 } { match unsafe { & (* this) . kind } { CountLatchKind :: Stealing { latch , registry , worker_index } => { let registry = Arc :: clone (registry) ; if unsafe { CoreLatch :: set (latch) } { registry . notify_worker_latch_is_set (* worker_index) ; } } CountLatchKind :: Blocking { latch } => unsafe { LockLatch :: set (latch) } , } } } }