use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : 'static > std :: ops :: Deref for AtomicRef < T > { type Target = T ; fn deref (& self) -> & Self :: Target { unsafe { & * self . 0 . load (Ordering :: SeqCst) } } }