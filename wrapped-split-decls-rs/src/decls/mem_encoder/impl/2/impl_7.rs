use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Encodable < MemEncoder > for IntEncodedWithFixedSize { # [inline] fn encode (& self , e : & mut MemEncoder) { let start_pos = e . position () ; e . write_array (self . 0 . to_le_bytes ()) ; let end_pos = e . position () ; debug_assert_eq ! ((end_pos - start_pos) , IntEncodedWithFixedSize :: ENCODED_SIZE) ; } }