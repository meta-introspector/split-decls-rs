use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Mul < Size > for u64 { type Output = Size ; # [inline] fn mul (self , size : Size) -> Size { size * self } }
}