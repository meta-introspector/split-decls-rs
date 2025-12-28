use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < NonZero < u8 > > for MixedUnit { # [inline] fn from (byte : NonZero < u8 >) -> Self { if byte . get () . is_ascii () { MixedUnit :: Char (NonZero :: new (byte . get () as char) . unwrap ()) } else { MixedUnit :: HighByte (byte) } } }