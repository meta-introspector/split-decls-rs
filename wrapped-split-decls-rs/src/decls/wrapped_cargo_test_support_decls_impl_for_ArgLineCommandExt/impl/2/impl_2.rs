use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ArgLineCommandExt for snapbox :: cmd :: Command { fn arg < S : AsRef < std :: ffi :: OsStr > > (self , s : S) -> Self { self . arg (s) } }