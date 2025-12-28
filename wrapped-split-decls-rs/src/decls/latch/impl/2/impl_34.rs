use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < L : Latch > Latch for LatchRef < '_ , L > { # [inline] unsafe fn set (this : * const Self) { unsafe { L :: set ((* this) . inner) } ; } }