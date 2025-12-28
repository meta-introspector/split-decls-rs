use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Deref for OwnedSlice { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { unsafe { & * self . bytes } } }