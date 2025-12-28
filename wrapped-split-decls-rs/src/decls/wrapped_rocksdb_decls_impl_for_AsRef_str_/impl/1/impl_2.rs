use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsRef < str > for Error { fn as_ref (& self) -> & str { & self . message } }
}