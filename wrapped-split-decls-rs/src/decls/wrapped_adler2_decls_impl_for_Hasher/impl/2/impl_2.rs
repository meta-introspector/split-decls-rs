use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Hasher for Adler32 { # [inline] fn finish (& self) -> u64 { u64 :: from (self . checksum ()) } fn write (& mut self , bytes : & [u8]) { self . write_slice (bytes) ; } }