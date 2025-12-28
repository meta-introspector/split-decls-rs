use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl TryFrom < u8 > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (byte : u8) -> Result < Self , EscapeError > { NonZero :: new (byte) . map (From :: from) . ok_or (EscapeError :: NulInCStr) } }
}