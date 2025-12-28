use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : ? Sized > FreezeWriteGuard < 'a , T > { # [inline] pub fn map < U : ? Sized > (mut this : Self , f : impl FnOnce (& mut T) -> & mut U ,) -> FreezeWriteGuard < 'a , U > { FreezeWriteGuard { data : NonNull :: from (f (& mut * this)) , _lock_guard : this . _lock_guard , frozen : this . frozen , marker : PhantomData , } } }