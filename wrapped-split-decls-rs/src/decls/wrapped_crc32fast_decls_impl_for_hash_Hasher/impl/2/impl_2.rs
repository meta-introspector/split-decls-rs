use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl hash :: Hasher for Hasher { fn write (& mut self , bytes : & [u8]) { self . update (bytes) } fn finish (& self) -> u64 { u64 :: from (self . clone () . finalize ()) } }