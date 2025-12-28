use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < P , T > Sync for TaggedRef < '_ , P , T > where P : Sync + Aligned + ? Sized , T : Sync + Tag , { }