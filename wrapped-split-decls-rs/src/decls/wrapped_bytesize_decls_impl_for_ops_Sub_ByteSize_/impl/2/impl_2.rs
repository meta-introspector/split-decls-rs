use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ops :: Sub < ByteSize > for ByteSize { type Output = ByteSize ; # [inline (always)] fn sub (self , rhs : ByteSize) -> ByteSize { ByteSize (self . 0 - rhs . 0) } }