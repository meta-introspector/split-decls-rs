use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : DynSync + DynSend > DynSync for FreezeLock < T > { }