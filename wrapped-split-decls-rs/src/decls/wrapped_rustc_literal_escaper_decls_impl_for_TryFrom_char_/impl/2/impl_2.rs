use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl TryFrom < char > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (c : char) -> Result < Self , EscapeError > { NonZero :: new (c) . map (MixedUnit :: Char) . ok_or (EscapeError :: NulInCStr) } }