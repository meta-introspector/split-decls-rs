use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Mul < u64 > for Size { type Output = Size ; # [inline] fn mul (self , count : u64) -> Size { match self . bytes () . checked_mul (count) { Some (bytes) => Size :: from_bytes (bytes) , None => panic ! ("Size::mul: {} * {} doesn't fit in u64" , self . bytes () , count) , } } }