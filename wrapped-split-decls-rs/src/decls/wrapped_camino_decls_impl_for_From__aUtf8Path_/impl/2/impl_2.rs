use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > From < & 'a Utf8Path > for Cow < 'a , Path > { fn from (path : & 'a Utf8Path) -> Cow < 'a , Path > { Cow :: Borrowed (path . as_ref ()) } }
}