use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl error :: Error for Error { fn description (& self) -> & str { & self . message } }
}