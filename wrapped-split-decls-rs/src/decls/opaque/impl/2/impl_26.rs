use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > Decodable < MemDecoder < 'a > > for IntEncodedWithFixedSize { # [inline] fn decode (decoder : & mut MemDecoder < 'a >) -> IntEncodedWithFixedSize { let bytes = decoder . read_array :: < { IntEncodedWithFixedSize :: ENCODED_SIZE } > () ; IntEncodedWithFixedSize (u64 :: from_le_bytes (bytes)) } }
}