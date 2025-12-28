use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Sub for Size { type Output = Size ; # [inline] fn sub (self , other : Size) -> Size { Size :: from_bytes (self . bytes () . checked_sub (other . bytes ()) . unwrap_or_else (| | { panic ! ("Size::sub: {} - {} would result in negative size" , self . bytes () , other . bytes ()) })) } }