use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl CheckRaw for CStr { type RawUnit = NonZero < char > ; # [inline] fn char2raw_unit (c : char) -> Result < Self :: RawUnit , EscapeError > { NonZero :: new (c) . ok_or (EscapeError :: NulInCStr) } }