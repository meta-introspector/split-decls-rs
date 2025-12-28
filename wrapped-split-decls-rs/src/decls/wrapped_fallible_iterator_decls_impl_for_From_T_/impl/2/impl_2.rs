use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , U > From < T > for MappedErr < T , U > { # [inline] fn from (t : T) -> MappedErr < T , U > { MappedErr :: It (t) } }
}