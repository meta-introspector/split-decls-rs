use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Internable + ? Sized > Drop for Interned < T > { # [inline] fn drop (& mut self) { if Arc :: count (& self . arc) == 2 { self . drop_slow () ; } } }