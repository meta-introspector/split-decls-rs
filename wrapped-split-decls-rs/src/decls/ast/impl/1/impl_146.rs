use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < AnonConst > for Term { fn from (v : AnonConst) -> Self { Term :: Const (v) } }
}