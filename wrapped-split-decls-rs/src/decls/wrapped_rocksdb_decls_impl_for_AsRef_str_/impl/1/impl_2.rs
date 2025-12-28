use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AsRef < str > for Error { fn as_ref (& self) -> & str { & self . message } }