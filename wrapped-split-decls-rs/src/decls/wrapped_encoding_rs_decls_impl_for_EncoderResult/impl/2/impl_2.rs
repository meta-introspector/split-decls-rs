use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl EncoderResult { fn unmappable_from_bmp (bmp : u16) -> EncoderResult { EncoderResult :: Unmappable (:: core :: char :: from_u32 (u32 :: from (bmp)) . unwrap ()) } }