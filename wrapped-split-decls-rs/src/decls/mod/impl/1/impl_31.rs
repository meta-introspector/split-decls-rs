use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > ScopePtr < T > { unsafe fn as_ref (& self) -> & T { unsafe { & * self . 0 } } }