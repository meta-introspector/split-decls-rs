use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Primitive > core :: panic :: RefUnwindSafe for AtomicMaybeUninit < T > { }