use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AsRef < OsStr > for Utf8PathBuf { # [inline] fn as_ref (& self) -> & OsStr { self . as_os_str () } }