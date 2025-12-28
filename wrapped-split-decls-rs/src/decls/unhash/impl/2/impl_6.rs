use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Hasher for Unhasher { # [inline] fn finish (& self) -> u64 { self . value } fn write (& mut self , _bytes : & [u8]) { unimplemented ! ("use write_u64") ; } # [inline] fn write_u64 (& mut self , value : u64) { debug_assert_eq ! (0 , self . value , "Unhasher doesn't mix values!") ; self . value = value ; } }