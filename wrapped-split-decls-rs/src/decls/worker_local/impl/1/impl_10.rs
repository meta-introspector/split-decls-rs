use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Deref for WorkerLocal < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { self . current () } }