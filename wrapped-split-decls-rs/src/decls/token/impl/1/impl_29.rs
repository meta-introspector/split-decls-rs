use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < bool > for IdentIsRaw { fn from (b : bool) -> Self { if b { Self :: Yes } else { Self :: No } } }
}