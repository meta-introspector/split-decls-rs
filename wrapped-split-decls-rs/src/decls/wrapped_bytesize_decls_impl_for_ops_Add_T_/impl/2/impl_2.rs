use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > ops :: Add < T > for ByteSize where T : Into < u64 > , { type Output = ByteSize ; # [inline (always)] fn add (self , rhs : T) -> ByteSize { ByteSize (self . 0 + (rhs . into ())) } }