use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " This gives access to a fresh parallel guard in the closure and will unwind any panics"] # [doc = " caught in it after the closure returns."] # [inline] pub fn parallel_guard < R > (f : impl FnOnce (& ParallelGuard) -> R) -> R { let guard = ParallelGuard { panic : Mutex :: new (None) } ; let ret = f (& guard) ; if let Some (IntoDynSyncSend (panic)) = guard . panic . into_inner () { resume_unwind (panic) ; } ret }