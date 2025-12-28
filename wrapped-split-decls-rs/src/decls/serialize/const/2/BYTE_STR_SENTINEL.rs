use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " For byte strings there are no bytes that cannot occur. Just use this value"] # [doc = " as a best-effort sentinel. There is no validation skipped so the potential"] # [doc = " for badness is lower than in the `STR_SENTINEL` case."] const BYTE_STR_SENTINEL : u8 = 0xC2 ;
}