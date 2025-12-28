use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Deref for Mmap { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { & self . 0 } }