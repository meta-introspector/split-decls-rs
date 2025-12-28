use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : RefCnt , S : Strategy < T > > Drop for ArcSwapAny < T , S > { fn drop (& mut self) { let ptr = * self . ptr . get_mut () ; unsafe { self . strategy . wait_for_readers (ptr , & self . ptr) ; T :: dec (ptr) ; } } }